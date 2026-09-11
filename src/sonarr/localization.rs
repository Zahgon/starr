use super::Sonarr;
use crate::error::Result;
use crate::helpers::str_val;
use crate::interface::ApiClientExt;
use crate::is_default;
use crate::req::{Request, path_join};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

const BP_LOCALIZATION: &str = "v3/localization";

/// Localization is the `/api/v3/localization` resource.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Localization {
    /// Localization ID.
    #[serde(default)]
    pub id: i32,
    /// The translated strings.
    #[serde(default, skip_serializing_if = "is_default")]
    pub strings: HashMap<String, String>,
}

/// UILanguage is an item from `/api/v3/localization/language`.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UILanguage {
    /// Identifier of the language.
    #[serde(default, skip_serializing_if = "is_default")]
    pub identifier: String,
}

impl Sonarr {
    /// Returns the default localization dictionary.
    pub async fn get_localization(&self) -> Result<Localization> {
        self.api.get_into(Request::new(BP_LOCALIZATION)).await
    }

    /// Returns a localization dictionary by id.
    pub async fn get_localization_by_id(&self, id: i32) -> Result<Localization> {
        self.api
            .get_into(Request::new(path_join(&[BP_LOCALIZATION, &str_val(id)])))
            .await
    }

    /// Returns available UI languages.
    pub async fn get_localization_languages(&self) -> Result<Vec<UILanguage>> {
        self.api
            .get_into(Request::new(path_join(&[BP_LOCALIZATION, "language"])))
            .await
    }
}
