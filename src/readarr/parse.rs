use super::{Author, Book, Readarr};
use crate::error::Result;
use crate::interface::ApiClientExt;
use crate::is_default;
use crate::req::Request;
use crate::values::Values;
use serde::{Deserialize, Serialize};

const BP_PARSE: &str = "v1/parse";

/// ParseOutput is returned from `GET /api/v1/parse`.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ParseOutput {
    /// Parse result ID.
    #[serde(default)]
    pub id: i64,
    /// The title that was parsed.
    #[serde(default, skip_serializing_if = "is_default")]
    pub title: String,
    /// The author the release belongs to.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub author: Option<Author>,
    /// The books the release covers.
    #[serde(default, skip_serializing_if = "is_default")]
    pub books: Vec<Book>,
    /// Custom formats matched by the release.
    #[serde(default, skip_serializing_if = "is_default")]
    pub custom_formats: Vec<serde_json::Value>,
    /// Total score of the matched custom formats.
    #[serde(default)]
    pub custom_format_score: i64,
}

impl Readarr {
    /// Resolves a release title into parsed book metadata.
    pub async fn parse(&self, title: &str) -> Result<ParseOutput> {
        let mut query = Values::new();
        query.set("title", title);

        self.api
            .get_into(Request::new(BP_PARSE).with_query(query))
            .await
    }
}
