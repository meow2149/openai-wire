//! Resource conveniences built on the same customizable request type.
use crate::{
    Client, Multipart, Page, Request, Result, Upload,
    client::encode_segment,
    types::{
        self,
        audio::{SpeechRequest, Transcription},
        batches::{Batch, BatchRequest},
        chat::{ChatCompletion, ChatRequest},
        embeddings::{EmbeddingRequest, Embeddings as EmbeddingResponse},
        files::File,
        images::{ImageRequest, Images as ImageResponse},
        models::Model,
        moderations::{Moderation, ModerationRequest},
        responses::{Response, ResponseRequest},
    },
};
use reqwest::Method;
use serde::Serialize;
use serde_json::Value;

/// A collection for APIs without a dedicated model, or an arbitrary relative resource.
/// JSON payloads can be replaced by application types with `Request::cast`.
#[derive(Clone, Debug)]
pub struct Resource {
    client: Client,
    path: String,
}
impl Resource {
    pub fn create(&self, body: impl Serialize) -> Request<Value> {
        self.client.request(Method::POST, &self.path).json(body)
    }
    pub fn list(&self) -> Request<Page<Value>> {
        self.client.request(Method::GET, &self.path)
    }
    pub fn retrieve(&self, id: &str) -> Request<Value> {
        self.item(Method::GET, id)
    }
    pub fn update(&self, id: &str, body: impl Serialize) -> Request<Value> {
        self.item(Method::POST, id).json(body)
    }
    pub fn delete(&self, id: &str) -> Request<Value> {
        self.item(Method::DELETE, id)
    }
    pub fn action(&self, id: &str, action: &str) -> Request<Value> {
        match (encode_segment(id), encode_segment(action)) {
            (Ok(id), Ok(action)) => self
                .client
                .request(Method::POST, format!("{}/{id}/{action}", self.path)),
            (Err(error), _) | (_, Err(error)) => {
                self.client.request(Method::POST, &self.path).fail(error)
            }
        }
    }
    pub fn child(&self, id: &str, collection: &str) -> Result<Self> {
        Ok(Self {
            client: self.client.clone(),
            path: format!(
                "{}/{}/{}",
                self.path,
                encode_segment(id)?,
                encode_segment(collection)?
            ),
        })
    }
    fn item(&self, method: Method, id: &str) -> Request<Value> {
        item_request(&self.client, method, &self.path, id, "")
    }
}

impl Client {
    pub fn resource(&self, path: impl Into<String>) -> Resource {
        Resource {
            client: self.clone(),
            path: path.into(),
        }
    }
    pub fn responses(&self) -> Responses<'_> {
        Responses(self)
    }
    pub fn chat(&self) -> Chat<'_> {
        Chat(self)
    }
    pub fn embeddings(&self) -> Embeddings<'_> {
        Embeddings(self)
    }
    pub fn images(&self) -> Images<'_> {
        Images(self)
    }
    pub fn audio(&self) -> Audio<'_> {
        Audio(self)
    }
    pub fn files(&self) -> Files<'_> {
        Files(self)
    }
    pub fn models(&self) -> Models<'_> {
        Models(self)
    }
    pub fn batches(&self) -> Batches<'_> {
        Batches(self)
    }
    pub fn moderations(&self) -> Moderations<'_> {
        Moderations(self)
    }
    pub fn conversations(&self) -> Resource {
        self.resource("/conversations")
    }
    pub fn vector_stores(&self) -> Resource {
        self.resource("/vector_stores")
    }
    pub fn containers(&self) -> Resource {
        self.resource("/containers")
    }
    pub fn fine_tuning(&self) -> Resource {
        self.resource("/fine_tuning/jobs")
    }
    pub fn evals(&self) -> Resource {
        self.resource("/evals")
    }
    pub fn uploads(&self) -> Resource {
        self.resource("/uploads")
    }
    pub fn skills(&self) -> Resource {
        self.resource("/skills")
    }
    pub fn admin(&self) -> Resource {
        self.resource("/organization")
    }
    pub fn webhook_endpoints(&self) -> Resource {
        self.resource("/webhook_endpoints")
    }

    /// Use a separate client for organization administration credentials.
    pub fn admin_from_env() -> Result<Self> {
        let key = std::env::var("OPENAI_ADMIN_KEY")
            .map_err(|_| crate::Error::Config("OPENAI_ADMIN_KEY is not set".into()))?;
        let mut builder = Self::builder().api_key(key);
        if let Ok(url) = std::env::var("OPENAI_BASE_URL") {
            builder = builder.base_url(url);
        }
        builder.build()
    }
}

fn item_request<T>(
    client: &Client,
    method: Method,
    path: &str,
    id: &str,
    suffix: &str,
) -> Request<T> {
    match encode_segment(id) {
        Ok(id) => client.request(method, format!("{path}/{id}{suffix}")),
        Err(error) => client.request(method, path).fail(error),
    }
}

pub struct Responses<'a>(&'a Client);
impl Responses<'_> {
    pub fn create(&self, body: ResponseRequest) -> Request<Response> {
        self.0.request(Method::POST, "/responses").json(body)
    }
    pub fn retrieve(&self, id: &str) -> Request<Response> {
        item_request(self.0, Method::GET, "/responses", id, "")
    }
    pub fn delete(&self, id: &str) -> Request<types::Deleted> {
        item_request(self.0, Method::DELETE, "/responses", id, "")
    }
    pub fn cancel(&self, id: &str) -> Request<Response> {
        item_request(self.0, Method::POST, "/responses", id, "/cancel")
    }
    pub fn input_items(&self, id: &str) -> Request<Page<Value>> {
        item_request(self.0, Method::GET, "/responses", id, "/input_items")
    }
    pub fn compact(&self, body: impl Serialize) -> Request<Value> {
        self.0
            .request(Method::POST, "/responses/compact")
            .json(body)
    }
    pub fn input_tokens(&self, body: impl Serialize) -> Request<Value> {
        self.0
            .request(Method::POST, "/responses/input_tokens")
            .json(body)
    }
}

pub struct Chat<'a>(&'a Client);
impl Chat<'_> {
    pub fn create(&self, body: ChatRequest) -> Request<ChatCompletion> {
        self.0.request(Method::POST, "/chat/completions").json(body)
    }
    pub fn list(&self) -> Request<Page<ChatCompletion>> {
        self.0.request(Method::GET, "/chat/completions")
    }
    pub fn retrieve(&self, id: &str) -> Request<ChatCompletion> {
        item_request(self.0, Method::GET, "/chat/completions", id, "")
    }
    pub fn update(&self, id: &str, body: impl Serialize) -> Request<ChatCompletion> {
        item_request(self.0, Method::POST, "/chat/completions", id, "").json(body)
    }
    pub fn delete(&self, id: &str) -> Request<types::Deleted> {
        item_request(self.0, Method::DELETE, "/chat/completions", id, "")
    }
    pub fn messages(&self, id: &str) -> Request<Page<types::chat::Message>> {
        item_request(self.0, Method::GET, "/chat/completions", id, "/messages")
    }
}

pub struct Embeddings<'a>(&'a Client);
impl Embeddings<'_> {
    pub fn create(&self, body: EmbeddingRequest) -> Request<EmbeddingResponse> {
        self.0.request(Method::POST, "/embeddings").json(body)
    }
}
pub struct Images<'a>(&'a Client);
impl Images<'_> {
    pub fn generate(&self, body: ImageRequest) -> Request<ImageResponse> {
        self.0
            .request(Method::POST, "/images/generations")
            .json(body)
    }
    pub fn edit(&self, form: Multipart) -> Request<ImageResponse> {
        self.0
            .request(Method::POST, "/images/edits")
            .multipart(form)
    }
}
pub struct Audio<'a>(&'a Client);
impl Audio<'_> {
    pub fn speech(&self, body: SpeechRequest) -> Request<Value> {
        self.0.request(Method::POST, "/audio/speech").json(body)
    }
    pub fn transcribe(&self, model: &str, file: Upload) -> Request<Transcription> {
        self.0
            .request(Method::POST, "/audio/transcriptions")
            .multipart(Multipart::new().text("model", model).file("file", file))
    }
    pub fn translate(&self, model: &str, file: Upload) -> Request<Transcription> {
        self.0
            .request(Method::POST, "/audio/translations")
            .multipart(Multipart::new().text("model", model).file("file", file))
    }
}
pub struct Files<'a>(&'a Client);
impl Files<'_> {
    pub fn upload(&self, purpose: &str, file: Upload) -> Request<File> {
        self.0
            .request(Method::POST, "/files")
            .multipart(Multipart::new().text("purpose", purpose).file("file", file))
    }
    pub fn list(&self) -> Request<Page<File>> {
        self.0.request(Method::GET, "/files")
    }
    pub fn retrieve(&self, id: &str) -> Request<File> {
        item_request(self.0, Method::GET, "/files", id, "")
    }
    pub fn delete(&self, id: &str) -> Request<types::Deleted> {
        item_request(self.0, Method::DELETE, "/files", id, "")
    }
    pub fn content(&self, id: &str) -> Request<Value> {
        item_request(self.0, Method::GET, "/files", id, "/content")
    }
}
pub struct Models<'a>(&'a Client);
impl Models<'_> {
    pub fn list(&self) -> Request<Page<Model>> {
        self.0.request(Method::GET, "/models")
    }
    pub fn retrieve(&self, id: &str) -> Request<Model> {
        item_request(self.0, Method::GET, "/models", id, "")
    }
    pub fn delete(&self, id: &str) -> Request<types::Deleted> {
        item_request(self.0, Method::DELETE, "/models", id, "")
    }
}
pub struct Batches<'a>(&'a Client);
impl Batches<'_> {
    pub fn create(&self, body: BatchRequest) -> Request<Batch> {
        self.0.request(Method::POST, "/batches").json(body)
    }
    pub fn list(&self) -> Request<Page<Batch>> {
        self.0.request(Method::GET, "/batches")
    }
    pub fn retrieve(&self, id: &str) -> Request<Batch> {
        item_request(self.0, Method::GET, "/batches", id, "")
    }
    pub fn cancel(&self, id: &str) -> Request<Batch> {
        item_request(self.0, Method::POST, "/batches", id, "/cancel")
    }
}
pub struct Moderations<'a>(&'a Client);
impl Moderations<'_> {
    pub fn create(&self, body: ModerationRequest) -> Request<Moderation> {
        self.0.request(Method::POST, "/moderations").json(body)
    }
}
