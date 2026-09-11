use super::Sonarr;
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
    /// Series the file belongs to.
    #[serde(default)]
    pub series_id: i64,
    /// Season the file belongs to.
    #[serde(default)]
    pub season_number: i64,
    /// Episodes the file covers.
    #[serde(default)]
    pub episode_numbers: Vec<i64>,
    /// File that would be renamed.
    #[serde(default)]
    pub episode_file_id: i64,
    /// Current path of the file.
    #[serde(default, skip_serializing_if = "is_default")]
    pub existing_path: String,
    /// Path the file would be renamed to.
    #[serde(default, skip_serializing_if = "is_default")]
    pub new_path: String,
}

impl Sonarr {
    /// Checks if the episodes in the specified series (database ID) and season
    /// need to be renamed to follow the naming format.
    ///
    /// If `season_number` is set to -1, it will check all seasons at once.
    pub async fn get_renames(&self, series_id: i64, season_number: i64) -> Result<Vec<Rename>> {
        let mut params = Values::new();
        params.set("seriesId", str_val(series_id));

        if season_number != -1 {
            params.set("seasonNumber", str_val(season_number));
        }

        self.api
            .get_into(Request::new(BP_RENAME).with_query(params))
            .await
    }

    /// Checks if the episodes in the specified series (database ID) need to be
    /// renamed to follow the naming format.
    pub async fn get_series_renames(&self, series_id: i64) -> Result<Vec<Rename>> {
        self.get_renames(series_id, -1).await
    }
}
