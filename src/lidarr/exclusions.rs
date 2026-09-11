use super::Lidarr;
use crate::error::{Error, Result};
use crate::helpers::str_val;
use crate::interface::ApiClientExt;
use crate::is_default;
use crate::req::{Request, path_join};
use serde::{Deserialize, Serialize};
use std::fmt::Write;

const BP_EXCLUSIONS: &str = "v1/importlistexclusion";

/// Exclusion is a Lidarr excluded item.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Exclusion {
    /// MusicBrainz ID of the excluded artist.
    #[serde(default)]
    pub foreign_id: String,
    /// Name of the excluded artist.
    #[serde(default)]
    pub artist_name: String,
    /// Exclusion ID.
    #[serde(default, skip_serializing_if = "is_default")]
    pub id: i64,
}

impl Lidarr {
    /// Returns all configured exclusions from Lidarr.
    pub async fn get_exclusions(&self) -> Result<Vec<Exclusion>> {
        self.api.get_into(Request::new(BP_EXCLUSIONS)).await
    }

    /// Changes an exclusion in Lidarr.
    pub async fn update_exclusion(&self, exclusion: &Exclusion) -> Result<Exclusion> {
        let req = Request::new(path_join(&[BP_EXCLUSIONS, &str_val(exclusion.id)]))
            .with_json(exclusion)?;
        self.api.put_into(req).await
    }

    /// Removes exclusions from Lidarr.
    ///
    /// Every ID is attempted; the failures are collected into one error.
    pub async fn delete_exclusions(&self, ids: &[i64]) -> Result<()> {
        let mut errs = String::new();

        for id in ids {
            let uri = path_join(&[BP_EXCLUSIONS, &str_val(*id)]);
            if let Err(err) = self.api.delete_any(Request::new(uri.clone())).await {
                let _ = write!(errs, "api.Post({uri}): {err} ");
            }
        }

        if !errs.is_empty() {
            return Err(Error::Request(errs));
        }

        Ok(())
    }

    /// Adds one exclusion to Lidarr.
    pub async fn add_exclusion(&self, exclusion: &Exclusion) -> Result<Exclusion> {
        let input = Exclusion {
            id: 0,
            ..exclusion.clone()
        };

        self.api
            .post_into(Request::new(BP_EXCLUSIONS).with_json(&input)?)
            .await
    }
}
