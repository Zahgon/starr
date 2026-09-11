use super::Radarr;
use crate::error::Result;
use crate::interface::ApiClientExt;
use crate::is_default;
use crate::req::Request;
use crate::string_enum;
use serde::{Deserialize, Serialize};

const BP_NAMING: &str = "v3/config/naming";

/// Naming represents the `config/naming` endpoint in Radarr.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Naming {
    /// Whether movie files are renamed.
    #[serde(default, skip_serializing_if = "is_default")]
    pub rename_movies: bool,
    /// Whether illegal characters are replaced.
    #[serde(default, skip_serializing_if = "is_default")]
    pub replace_illegal_characters: bool,
    /// Config ID. Must always be 1 (Oct 10, 2022).
    #[serde(default)]
    pub id: i64,
    /// How colons are replaced in file names.
    #[serde(default, skip_serializing_if = "is_default")]
    pub colon_replacement_format: Crf,
    /// Format for standard movie files. Required.
    #[serde(default)]
    pub standard_movie_format: String,
    /// Format for movie folders. Required.
    #[serde(default)]
    pub movie_folder_format: String,
}

string_enum! {
    /// Crf is ColonReplacementFormat, for naming config.
    pub struct Crf {
        /// Remove the colon.
        const COLON_DELETE = "delete";
        /// Replace the colon with a dash.
        const COLON_REPLACE_WITH_DASH = "dash";
        /// Replace the colon with a space and a dash.
        const COLON_REPLACE_WITH_SPACE_DASH = "spaceDash";
        /// Replace the colon with a space, a dash and a space.
        const COLON_REPLACE_WITH_SPACE_DASH_SPACE = "spaceDashSpace";
    }
}

impl Radarr {
    /// Returns the file naming rules.
    pub async fn get_naming(&self) -> Result<Naming> {
        self.api.get_into(Request::new(BP_NAMING)).await
    }

    /// Updates the file naming rules.
    pub async fn update_naming(&self, naming: &Naming) -> Result<Naming> {
        let naming = Naming {
            id: 1,
            ..naming.clone()
        };

        self.api
            .put_into(Request::new(BP_NAMING).with_json(&naming)?)
            .await
    }
}
