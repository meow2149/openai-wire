use crate::{Error, Request};
use futures_core::Stream;
use serde::{Deserialize, Serialize, de::DeserializeOwned};
use serde_json::{Map, Value};
use std::collections::HashSet;

/// Cursor-based list response. Unknown fields are retained.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Page<T> {
    pub data: Vec<T>,
    #[serde(default)]
    pub object: Option<String>,
    #[serde(default)]
    pub has_more: bool,
    #[serde(default)]
    pub first_id: Option<String>,
    #[serde(default)]
    pub last_id: Option<String>,
    #[serde(default)]
    pub next_page: Option<String>,
    #[serde(default)]
    pub next: Option<String>,
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

impl<T: DeserializeOwned + Send + 'static> Request<Page<T>> {
    /// Fetch items lazily across cursor pages. Detect repeated cursors instead of looping.
    /// Supports `after`/`last_id` and administrative `page`/`next_page` pagination.
    pub fn items(self) -> impl Stream<Item = crate::Result<T>> + Send {
        let parameter =
            if self.path.starts_with("/organization/") || self.path.starts_with("/projects/") {
                "page"
            } else {
                "after"
            };
        self.items_with_cursor(parameter)
    }

    /// Override the query parameter used for `next` or item-ID cursors.
    /// Use `page` for token pagination on a custom administrative path.
    pub fn items_with_cursor(
        mut self,
        parameter: &str,
    ) -> impl Stream<Item = crate::Result<T>> + Send {
        let parameter = parameter.to_owned();
        async_stream::try_stream! {
            if let Some(error) = self.take_error() { Err(error)?; }
            let mut seen = HashSet::new();
            for (key, value) in &self.query { if key == "after" || key == "page" || key == &parameter { seen.insert((key.clone(), value.clone())); } }
            loop {
                let page = self.duplicate().cast::<Page<Value>>().send().await?;
                let cursor = if page.has_more {
                    if let Some(cursor) = page.next_page { Some(("page".to_owned(), cursor)) }
                    else if let Some(cursor) = page.next.or(page.last_id)
                        .or_else(|| page.data.last().and_then(|item| item.get("id")).and_then(Value::as_str).map(str::to_owned)) {
                        Some((parameter.clone(), cursor))
                    }
                    else { Err(Error::Stream("paginated response has_more but no cursor".into()))?; None }
                } else { None };
                for item in page.data { yield serde_json::from_value::<T>(item)?; }
                let Some((name, cursor)) = cursor else { break; };
                if cursor.is_empty() || !seen.insert((name.clone(), cursor.clone())) { Err(Error::Stream("pagination repeated or returned an empty cursor".into()))?; }
                self.query.retain(|(key, _)| key != "after" && key != "before" && key != "page" && key != &parameter);
                self.query.push((name, cursor));
            }
        }
    }
}
