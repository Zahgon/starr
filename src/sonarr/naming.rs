use super::Sonarr;
use crate::error::Result;
use crate::interface::ApiClientExt;
use crate::req::Request;
use serde::{Deserialize, Serialize};

const BP_NAMING: &str = "v3/config/naming";

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
/// Replace the colon with the custom replacement format.
pub const CUSTOM: CRF = CRF(5);

/// Naming represents the `config/naming` endpoint in Sonarr.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Naming {
    /// Whether episodes are renamed on import.
    #[serde(default)]
    pub rename_episodes: bool,
    /// Whether illegal characters are replaced instead of removed.
    #[serde(default)]
    pub replace_illegal_characters: bool,
    /// How colons are replaced.
    #[serde(default)]
    pub colon_replacement_format: CRF,
    /// Config ID.
    #[serde(default)]
    pub id: i64,
    /// Style used when a file holds several episodes.
    #[serde(default)]
    pub multi_episode_style: i64,
    /// Format used for daily episodes.
    #[serde(default)]
    pub daily_episode_format: String,
    /// Format used for anime episodes.
    #[serde(default)]
    pub anime_episode_format: String,
    /// Format used for series folders.
    #[serde(default)]
    pub series_folder_format: String,
    /// Format used for season folders.
    #[serde(default)]
    pub season_folder_format: String,
    /// Format used for the specials folder.
    #[serde(default)]
    pub specials_folder_format: String,
    /// Format used for standard episodes.
    #[serde(default)]
    pub standard_episode_format: String,
    /// Replacement used when the colon format is custom.
    #[serde(default)]
    pub custom_colon_replacement_format: String,
}

impl Sonarr {
    /// Returns the naming.
    pub async fn get_naming(&self) -> Result<Naming> {
        self.api.get_into(Request::new(BP_NAMING)).await
    }

    /// Updates the naming.
    pub async fn update_naming(&self, naming: &Naming) -> Result<Naming> {
        self.api
            .put_into(Request::new(BP_NAMING).with_json(naming)?)
            .await
    }
}
