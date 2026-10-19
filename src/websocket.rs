//! JSON WebSockets for Realtime, Responses, and compatible relative endpoints.
//! Connections do not reconnect automatically: callers own session recovery.
use crate::{ApiResponse, Client, Error, Result};
use base64::{Engine, engine::general_purpose::STANDARD};
use futures_core::Stream;
use futures_util::{
    SinkExt, StreamExt,
    stream::{SplitSink, SplitStream},
};
use reqwest::header::HeaderMap;
use serde::{Serialize, de::DeserializeOwned};
use serde_json::{Value, json};
use std::{
    pin::Pin,
    task::{Context, Poll},
};
use tokio::net::TcpStream;
use tokio_tungstenite::{
    MaybeTlsStream, WebSocketStream,
    tungstenite::{
        Error as SocketError, Message, client::IntoClientRequest, protocol::WebSocketConfig,
    },
};

type Socket = WebSocketStream<MaybeTlsStream<TcpStream>>;

impl From<SocketError> for Error {
    fn from(error: SocketError) -> Self {
        Self::WebSocket(Box::new(error))
    }
}

impl Client {
    /// Connect a relative JSON WebSocket endpoint using this client's credentials.
    /// HTTP proxy and custom TLS settings on `http_client` do not apply to WebSockets.
    pub async fn connect_websocket(
        &self,
        path: &str,
        query: &[(&str, &str)],
    ) -> Result<ApiResponse<Connection>> {
        let mut url = self.url(path)?;
        let scheme = if url.scheme() == "https" { "wss" } else { "ws" };
        url.set_scheme(scheme)
            .map_err(|_| Error::Config("invalid WebSocket URL scheme".into()))?;
        url.query_pairs_mut()
            .extend_pairs(&self.inner.query)
            .extend_pairs(query);
        let mut request = url.as_str().into_client_request()?;
        request.headers_mut().extend(self.inner.headers.clone());
        // OpenAI can emit large audio and tool events. Do not impose a new SDK limit.
        let mut config = WebSocketConfig::default();
        config.max_message_size = None;
        config.max_frame_size = None;
        let (socket, response) = tokio::time::timeout(
            self.inner.timeout,
            tokio_tungstenite::connect_async_with_config(request, Some(config), false),
        )
        .await
        .map_err(|_| Error::Stream("WebSocket handshake timed out".into()))??;
        let headers = response.headers().clone();
        let status = response.status();
        Ok(ApiResponse {
            data: Connection {
                socket,
                headers: headers.clone(),
                ended: false,
            },
            status,
            headers,
        })
    }
    pub async fn realtime(&self, model: &str) -> Result<Connection> {
        if model.is_empty() {
            return Err(Error::InvalidRequest("model is empty".into()));
        }
        Ok(self
            .connect_websocket("/realtime", &[("model", model)])
            .await?
            .data)
    }
    pub async fn responses_websocket(&self) -> Result<Connection> {
        Ok(self.connect_websocket("/responses", &[]).await?.data)
    }
}

/// A bidirectional JSON connection. Use `split` for concurrent sending and receiving.
pub struct Connection {
    socket: Socket,
    pub headers: HeaderMap,
    ended: bool,
}
impl std::fmt::Debug for Connection {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Connection").finish_non_exhaustive()
    }
}
impl Connection {
    pub async fn send(&mut self, event: impl Serialize) -> Result<()> {
        self.socket
            .send(Message::Text(serde_json::to_string(&event)?.into()))
            .await?;
        Ok(())
    }
    pub async fn receive<T: DeserializeOwned>(&mut self) -> Result<Option<T>> {
        match self.next().await {
            Some(value) => Ok(Some(serde_json::from_value(value?)?)),
            None => Ok(None),
        }
    }
    pub async fn update_session(&mut self, session: impl Serialize) -> Result<()> {
        self.send(json!({"type": "session.update", "session": session}))
            .await
    }
    pub async fn append_audio(&mut self, audio: &[u8]) -> Result<()> {
        self.send(json!({"type": "input_audio_buffer.append", "audio": STANDARD.encode(audio)}))
            .await
    }
    pub async fn close(mut self) -> Result<()> {
        self.socket.close(None).await?;
        Ok(())
    }
    pub fn split(self) -> (Sender, Receiver) {
        let (sink, stream) = self.socket.split();
        (
            Sender(sink),
            Receiver {
                stream,
                ended: self.ended,
            },
        )
    }
}
impl Stream for Connection {
    type Item = Result<Value>;
    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        let Self { socket, ended, .. } = &mut *self;
        poll_json(socket, ended, cx)
    }
}

pub struct Sender(SplitSink<Socket, Message>);
impl Sender {
    pub async fn send(&mut self, event: impl Serialize) -> Result<()> {
        self.0
            .send(Message::Text(serde_json::to_string(&event)?.into()))
            .await?;
        Ok(())
    }
    pub async fn close(&mut self) -> Result<()> {
        self.0.close().await?;
        Ok(())
    }
}
pub struct Receiver {
    stream: SplitStream<Socket>,
    ended: bool,
}
impl Stream for Receiver {
    type Item = Result<Value>;
    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        let Self { stream, ended } = &mut *self;
        poll_json(stream, ended, cx)
    }
}

fn poll_json(
    stream: &mut (impl Stream<Item = std::result::Result<Message, SocketError>> + Unpin),
    ended: &mut bool,
    cx: &mut Context<'_>,
) -> Poll<Option<Result<Value>>> {
    if *ended {
        return Poll::Ready(None);
    }
    loop {
        match Pin::new(&mut *stream).poll_next(cx) {
            Poll::Ready(Some(Ok(Message::Text(text)))) => {
                return Poll::Ready(Some(serde_json::from_str(&text).map_err(Error::from)));
            }
            Poll::Ready(Some(Ok(Message::Binary(bytes)))) => {
                return Poll::Ready(Some(serde_json::from_slice(&bytes).map_err(Error::from)));
            }
            Poll::Ready(Some(Ok(Message::Close(_)))) | Poll::Ready(None) => {
                *ended = true;
                return Poll::Ready(None);
            }
            Poll::Ready(Some(Ok(_))) => continue,
            Poll::Ready(Some(Err(error))) => {
                *ended = true;
                return Poll::Ready(Some(Err(error.into())));
            }
            Poll::Pending => return Poll::Pending,
        }
    }
}
