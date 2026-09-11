use super::Radarr;
use crate::error::Result;
use crate::helpers::str_val;
use crate::interface::ApiClientExt;
use crate::is_default;
use crate::req::Request;
use crate::values::Values;
use serde::{Deserialize, Serialize};

const BP_RENAME: &str = "v3/rename";

/// Rename is the `/api/v3/rename` endpoint.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Rename {
    /// Rename item ID.
    #[serde(default)]
    pub id: i64,
    /// Movie the file belongs to.
    #[serde(default)]
    pub movie_id: i64,
    /// File that would be renamed.
    #[serde(default)]
    pub movie_file_id: i64,
    /// Current path of the file.
    #[serde(default, skip_serializing_if = "is_default")]
    pub existing_path: String,
    /// Path the file would be renamed to.
    #[serde(default, skip_serializing_if = "is_default")]
    pub new_path: String,
}

impl Radarr {
    /// Checks if the movie with the specified `movie_id` (database ID) needs to
    /// be renamed to follow the naming format.
    pub async fn get_renames(&self, movie_id: i64) -> Result<Vec<Rename>> {
        let mut params = Values::new();
        params.set("movieId", str_val(movie_id));

        self.api
            .get_into(Request::new(BP_RENAME).with_query(params))
            .await
    }
}
