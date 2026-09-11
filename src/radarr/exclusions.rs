use super::Radarr;
use crate::error::{Error, Result};
use crate::helpers::str_val;
use crate::interface::ApiClientExt;
use crate::is_default;
use crate::req::{Request, path_join};
use serde::{Deserialize, Serialize};
use std::fmt::Write;

const BP_EXCLUSIONS: &str = "v3/exclusions";

/// Exclusion is a Radarr excluded item.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Exclusion {
    /// TMDb ID of the excluded movie.
    #[serde(default, rename = "tmdbId")]
    pub tmdb_id: i64,
    /// Title of the excluded movie.
    #[serde(default, rename = "movieTitle")]
    pub title: String,
    /// Release year of the excluded movie.
    #[serde(default, rename = "movieYear")]
    pub year: i32,
    /// Exclusion ID.
    #[serde(default, skip_serializing_if = "is_default")]
    pub id: i64,
}

impl Radarr {
    /// Returns all configured exclusions from Radarr.
    pub async fn get_exclusions(&self) -> Result<Vec<Exclusion>> {
        self.api.get_into(Request::new(BP_EXCLUSIONS)).await
    }

    /// Changes an exclusion in Radarr.
    pub async fn update_exclusion(&self, exclusion: &Exclusion) -> Result<Exclusion> {
        let req =
            Request::new(path_join(&[BP_EXCLUSIONS, &str_val(exclusion.id)])).with_json(exclusion)?;
        self.api.put_into(req).await
    }

    /// Removes exclusions from Radarr.
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

    /// Adds multiple exclusions to Radarr.
    pub async fn add_exclusions(&self, exclusions: &[Exclusion]) -> Result<()> {
        let exclusions: Vec<Exclusion> = exclusions
            .iter()
            .map(|exclusion| Exclusion {
                id: 0,
                ..exclusion.clone()
            })
            .collect();

        let req = Request::new(path_join(&[BP_EXCLUSIONS, "bulk"])).with_json(&exclusions)?;
        self.api.post_any(req).await
    }

    /// Adds one exclusion to Radarr.
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
