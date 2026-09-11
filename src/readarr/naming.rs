use super::Readarr;
use crate::error::Result;
use crate::interface::ApiClientExt;
use crate::req::Request;
use serde::{Deserialize, Serialize};

const BP_NAMING: &str = "v1/config/naming";

/// CRF is ColonReplacementFormat, for naming config.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(transparent)]
pub struct CRF(pub i32);

/// Delete the colon.
pub const COLON_DELETE: CRF = CRF(0);
/// Replace the colon with a dash.
pub const COLON_REPLACE_WITH_DASH: CRF = CRF(1);
/// Replace the colon with a space and a dash.
pub const COLON_REPLACE_WITH_SPACE_DASH: CRF = CRF(2);
/// Replace the colon with a space, a dash and a space.
pub const COLON_REPLACE_WITH_SPACE_DASH_SPACE: CRF = CRF(3);
/// Replace the colon depending on the surrounding characters.
pub const COLON_SMART_REPLACE: CRF = CRF(4);

/// Naming represents the `config/naming` endpoint in Readarr.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Naming {
    /// Whether books are renamed on import.
    #[serde(default)]
    pub rename_books: bool,
    /// Whether illegal characters are replaced instead of removed.
    #[serde(default)]
    pub replace_illegal_characters: bool,
    /// Whether the author name is included in the file name.
    #[serde(default)]
    pub include_author_name: bool,
    /// Whether the book title is included in the file name.
    #[serde(default)]
    pub include_book_title: bool,
    /// Whether the quality is included in the file name.
    #[serde(default)]
    pub include_quality: bool,
    /// Whether spaces are replaced.
    #[serde(default)]
    pub replace_spaces: bool,
    /// How colons are replaced.
    #[serde(default)]
    pub colon_replacement_format: CRF,
    /// Config ID.
    #[serde(default)]
    pub id: i64,
    /// Format used for standard books.
    #[serde(default)]
    pub standard_book_format: String,
    /// Format used for author folders.
    #[serde(default)]
    pub author_folder_format: String,
}

impl Readarr {
    /// Returns the file naming rules.
    pub async fn get_naming(&self) -> Result<Naming> {
        self.api.get_into(Request::new(BP_NAMING)).await
    }

    /// Updates the file naming rules.
    pub async fn update_naming(&self, naming: &Naming) -> Result<Naming> {
        self.api
            .put_into(Request::new(BP_NAMING).with_json(naming)?)
            .await
    }
}
