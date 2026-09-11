use super::Radarr;
use crate::error::Result;
use crate::helpers::str_val;
use crate::interface::ApiClientExt;
use crate::is_default;
use crate::req::{Request, path_join};
use serde::{Deserialize, Serialize};

const BP_LANGUAGE: &str = "v3/language";

/// Language is an item from `/api/v3/language`.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Language {
    /// Language ID.
    #[serde(default)]
    pub id: i32,
    /// Language name.
    #[serde(default, skip_serializing_if = "is_default")]
    pub name: String,
}

impl Radarr {
    /// Returns all movie languages.
    pub async fn get_languages(&self) -> Result<Vec<Language>> {
        self.api.get_into(Request::new(BP_LANGUAGE)).await
    }

    /// Returns a single language by id.
    pub async fn get_language(&self, id: i32) -> Result<Language> {
        self.api
            .get_into(Request::new(path_join(&[BP_LANGUAGE, &str_val(id)])))
            .await
    }
}
