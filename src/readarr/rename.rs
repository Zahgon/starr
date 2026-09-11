use super::Readarr;
use crate::error::Result;
use crate::helpers::str_val;
use crate::interface::ApiClientExt;
use crate::is_default;
use crate::req::Request;
use crate::values::Values;
use serde::{Deserialize, Serialize};

const BP_RENAME: &str = "v1/rename";

/// Rename is the `/api/v1/rename` endpoint.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Rename {
    /// Rename item ID.
    #[serde(default)]
    pub id: i64,
    /// Author the file belongs to.
    #[serde(default)]
    pub author_id: i64,
    /// Book the file belongs to.
    #[serde(default)]
    pub book_id: i64,
    /// File that would be renamed.
    #[serde(default)]
    pub book_file_id: i64,
    /// Current path of the file.
    #[serde(default, skip_serializing_if = "is_default")]
    pub existing_path: String,
    /// Path the file would be renamed to.
    #[serde(default, skip_serializing_if = "is_default")]
    pub new_path: String,
}

impl Readarr {
    /// Checks if the specified book (database ID) from the author (database ID)
    /// needs to be renamed to follow the naming format.
    ///
    /// If `book_id` is set to -1, it will check all books at once.
    pub async fn get_renames(&self, author_id: i64, book_id: i64) -> Result<Vec<Rename>> {
        let mut params = Values::new();
        params.set("authorId", str_val(author_id));

        if book_id != -1 {
            params.set("bookId", str_val(book_id));
        }

        self.api
            .get_into(Request::new(BP_RENAME).with_query(params))
            .await
    }

    /// Checks if the books from the specified author (database ID) need to be
    /// renamed to follow the naming format.
    pub async fn get_author_renames(&self, author_id: i64) -> Result<Vec<Rename>> {
        self.get_renames(author_id, -1).await
    }
}
