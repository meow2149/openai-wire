use crate::{Error, Result};
use bytes::Bytes;
use reqwest::multipart::{Form, Part};
use std::path::{Path, PathBuf};

/// A replayable file upload. Disk files are reopened for every retry and streamed.
#[derive(Clone)]
pub struct Upload {
    source: Source,
    filename: String,
    mime: Option<String>,
}
#[derive(Clone)]
enum Source {
    Bytes(Bytes),
    Path(PathBuf),
}

impl Upload {
    pub fn bytes(filename: impl Into<String>, data: impl Into<Bytes>) -> Self {
        Self {
            source: Source::Bytes(data.into()),
            filename: filename.into(),
            mime: None,
        }
    }
    pub fn path(path: impl AsRef<Path>) -> Result<Self> {
        let path = path.as_ref();
        let filename = path
            .file_name()
            .and_then(|s| s.to_str())
            .ok_or_else(|| Error::InvalidRequest("file has no UTF-8 filename".into()))?;
        Ok(Self {
            source: Source::Path(path.into()),
            filename: filename.into(),
            mime: None,
        })
    }
    pub fn filename(mut self, name: impl Into<String>) -> Self {
        self.filename = name.into();
        self
    }
    pub fn mime(mut self, mime: impl Into<String>) -> Self {
        self.mime = Some(mime.into());
        self
    }
    async fn part(&self) -> Result<Part> {
        let part = match &self.source {
            Source::Bytes(bytes) => Part::stream_with_length(bytes.clone(), bytes.len() as u64),
            Source::Path(path) => {
                let file = tokio::fs::File::open(path).await?;
                let length = file.metadata().await?.len();
                let stream = tokio_util::io::ReaderStream::new(file);
                Part::stream_with_length(reqwest::Body::wrap_stream(stream), length)
            }
        }
        .file_name(self.filename.clone());
        match &self.mime {
            Some(mime) => Ok(part.mime_str(mime)?),
            None => Ok(part),
        }
    }
}

/// Replayable multipart fields, including repeated file and text fields.
#[derive(Clone, Default)]
pub struct Multipart {
    fields: Vec<(String, Field)>,
}
#[derive(Clone)]
enum Field {
    Text(String),
    File(Upload),
}

impl Multipart {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn text(mut self, name: impl Into<String>, value: impl Into<String>) -> Self {
        self.fields.push((name.into(), Field::Text(value.into())));
        self
    }
    pub fn file(mut self, name: impl Into<String>, file: Upload) -> Self {
        self.fields.push((name.into(), Field::File(file)));
        self
    }
    pub(crate) fn set_text(&mut self, name: String, value: String) {
        self.fields.retain(|(key, _)| key != &name);
        self.fields.push((name, Field::Text(value)));
    }
    pub(crate) async fn form(&self) -> Result<Form> {
        let mut form = Form::new();
        for (name, field) in &self.fields {
            form = match field {
                Field::Text(v) => form.text(name.clone(), v.clone()),
                Field::File(v) => form.part(name.clone(), v.part().await?),
            };
        }
        Ok(form)
    }
}
