use super::Sonarr;
use crate::error::{Error, Result};
use crate::helpers::str_val;
use crate::interface::ApiClientExt;
use crate::is_default;
use crate::req::{Request, path_join};
use serde::{Deserialize, Serialize};
use std::fmt::Write;

const BP_EXCLUSIONS: &str = "v3/importlistexclusion";

/// Exclusion is a Sonarr excluded item.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Exclusion {
    /// TheTVDB ID of the excluded series.
    #[serde(default, rename = "tvdbId")]
    pub tvdb_id: i64,
    /// Title of the excluded series.
    #[serde(default)]
    pub title: String,
    /// Exclusion ID.
    #[serde(default, skip_serializing_if = "is_default")]
    pub id: i64,
}

impl Sonarr {
    /// Returns all configured exclusions from Sonarr.
    pub async fn get_exclusions(&self) -> Result<Vec<Exclusion>> {
        self.api.get_into(Request::new(BP_EXCLUSIONS)).await
    }

    /// Changes an exclusion in Sonarr.
    pub async fn update_exclusion(&self, exclusion: &Exclusion) -> Result<Exclusion> {
        let req =
            Request::new(path_join(&[BP_EXCLUSIONS, &str_val(exclusion.id)])).with_json(exclusion)?;
        self.api.put_into(req).await
    }

    /// Removes exclusions from Sonarr.
    pub async fn delete_exclusions(&self, ids: &[i64]) -> Result<()> {
        let mut errs = String::new();

        for id in ids {
            let uri = path_join(&[BP_EXCLUSIONS, &str_val(*id)]);
            if let Err(err) = self.api.delete_any(Request::new(uri.clone())).await {
                let _ = write!(errs, "api.Post({uri}): {err} ");
            }
        }

        if errs.is_empty() {
            return Ok(());
        }

        Err(Error::Request(errs))
    }

    /// Adds one exclusion to Sonarr.
    pub async fn add_exclusion(&self, exclusion: &Exclusion) -> Result<Exclusion> {
        let exclusion = Exclusion {
            id: 0,
            ..exclusion.clone()
        };

        self.api
            .post_into(Request::new(BP_EXCLUSIONS).with_json(&exclusion)?)
            .await
    }
}
