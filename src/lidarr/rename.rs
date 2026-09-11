use super::Lidarr;
use crate::error::Result;
use crate::helpers::str_val;
use crate::interface::ApiClientExt;
use crate::is_default;
use crate::req::Request;
use crate::values::Values;
use serde::{Deserialize, Serialize};

const BP_RENAME: &str = "v1/rename";

/// Rename is the `/api/v1/rename` endpoint.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Rename {
    /// Rename ID.
    #[serde(default)]
    pub id: i64,
    /// Artist this rename belongs to.
    #[serde(default)]
    pub artist_id: i64,
    /// Album this rename belongs to.
    #[serde(default)]
    pub album_id: i64,
    /// Track numbers affected by this rename.
    #[serde(default)]
    pub track_numbers: Vec<i64>,
    /// Track file being renamed.
    #[serde(default)]
    pub track_file_id: i64,
    /// Path the file has today.
    #[serde(default, skip_serializing_if = "is_default")]
    pub existing_path: String,
    /// Path the file would have after the rename.
    #[serde(default, skip_serializing_if = "is_default")]
    pub new_path: String,
}

impl Lidarr {
    /// Checks if the tracks by the specified artist (database ID) on the specified
    /// album (database ID) need to be renamed to follow the naming format.
    ///
    /// If `album_id` is set to `-1`, it will check all albums at once.
    pub async fn get_renames(&self, artist_id: i64, album_id: i64) -> Result<Vec<Rename>> {
        let mut query = Values::new();
        query.set("artistId", str_val(artist_id));

        if album_id != -1 {
            query.set("albumId", str_val(album_id));
        }

        self.api
            .get_into(Request::new(BP_RENAME).with_query(query))
            .await
    }

    /// Checks if the tracks by the specified artist (database ID) need to be
    /// renamed to follow the naming format.
    pub async fn get_artist_renames(&self, artist_id: i64) -> Result<Vec<Rename>> {
        self.get_renames(artist_id, -1).await
    }
}
