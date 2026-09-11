use super::Sonarr;
use crate::error::Result;
use crate::helpers::str_val;
use crate::interface::ApiClientExt;
use crate::is_default;
use crate::req::{Request, path_join};
use serde::{Deserialize, Serialize};

const BP_LANGUAGE: &str = "v3/language";

/// AudioLanguage is an item from `/api/v3/language` (episode audio languages).
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AudioLanguage {
    /// Language ID.
    #[serde(default)]
    pub id: i32,
    /// Language name.
    #[serde(default, skip_serializing_if = "is_default")]
    pub name: String,
    /// Lowercase language name.
    #[serde(default, skip_serializing_if = "is_default")]
    pub name_lower: String,
}

impl Sonarr {
    /// Returns all languages from the `/api/v3/language` endpoint.
    pub async fn get_audio_languages(&self) -> Result<Vec<AudioLanguage>> {
        self.api.get_into(Request::new(BP_LANGUAGE)).await
    }

    /// Returns a single language by id.
    pub async fn get_audio_language(&self, id: i32) -> Result<AudioLanguage> {
        self.api
            .get_into(Request::new(path_join(&[BP_LANGUAGE, &str_val(id)])))
            .await
    }
}
