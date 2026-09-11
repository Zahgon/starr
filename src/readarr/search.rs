use super::{Author, Book, Readarr};
use crate::error::Result;
use crate::interface::ApiClientExt;
use crate::is_default;
use crate::req::Request;
use crate::values::Values;
use serde::{Deserialize, Serialize};

const BP_SEARCH: &str = "v1/search";

/// SearchResult is the struct returned from the `/api/v1/search` endpoint.
///
/// ID in this context means the index of the search result, not the book's ID.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SearchResult {
    /// Foreign ID of the result.
    #[serde(default, skip_serializing_if = "is_default")]
    pub foreign_id: String,
    /// The author, when the result is an author.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub author: Option<Author>,
    /// The book, when the result is a book.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub book: Option<Book>,
    /// Index of this search result.
    #[serde(default, skip_serializing_if = "is_default")]
    pub id: i32,
}

impl Readarr {
    /// Returns the search results for a term.
    pub async fn search(&self, term: &str) -> Result<Vec<SearchResult>> {
        if term.is_empty() {
            return Ok(Vec::new());
        }

        let mut params = Values::new();
        params.set("term", term);

        self.api
            .get_into(Request::new(BP_SEARCH).with_query(params))
            .await
    }
}
