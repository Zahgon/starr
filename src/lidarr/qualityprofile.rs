use super::Lidarr;
use crate::error::Result;
use crate::helpers::str_val;
use crate::interface::ApiClientExt;
use crate::req::{Request, path_join};
use crate::shared::{FormatItem, Quality};
use serde::{Deserialize, Serialize};

const BP_QUALITY_PROFILE: &str = "v1/qualityProfile";

/// QualityProfile is the `/api/v1/qualityprofile` endpoint.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct QualityProfile {
    /// Profile ID.
    #[serde(default)]
    pub id: i64,
    /// Profile name.
    #[serde(default)]
    pub name: String,
    /// Whether upgrades are allowed.
    #[serde(default)]
    pub upgrade_allowed: bool,
    /// Quality ID that stops upgrades.
    #[serde(default)]
    pub cutoff: i64,
    /// Qualities in this profile.
    #[serde(default, rename = "items")]
    pub qualities: Vec<Quality>,
    /// Minimum custom format score.
    #[serde(default)]
    pub min_format_score: i64,
    /// Minimum custom format score to upgrade.
    #[serde(default)]
    pub min_upgrade_format_score: i64,
    /// Custom format score that stops upgrades.
    #[serde(default)]
    pub cutoff_format_score: i64,
    /// Custom format scores in this profile.
    #[serde(default)]
    pub format_items: Vec<FormatItem>,
}

impl Lidarr {
    /// Returns the quality profiles.
    pub async fn get_quality_profiles(&self) -> Result<Vec<QualityProfile>> {
        self.api.get_into(Request::new(BP_QUALITY_PROFILE)).await
    }

    /// Returns a single quality profile.
    pub async fn get_quality_profile(&self, profile_id: i64) -> Result<QualityProfile> {
        self.api
            .get_into(Request::new(path_join(&[
                BP_QUALITY_PROFILE,
                &str_val(profile_id),
            ])))
            .await
    }

    /// Creates a quality profile and returns the new ID.
    pub async fn add_quality_profile(&self, profile: &QualityProfile) -> Result<i64> {
        let mut profile = profile.clone();
        profile.id = 0;

        let output: QualityProfile = self
            .api
            .post_into(Request::new(BP_QUALITY_PROFILE).with_json(&profile)?)
            .await?;

        Ok(output.id)
    }

    /// Updates a quality profile in place.
    pub async fn update_quality_profile(
        &self,
        profile: &QualityProfile,
    ) -> Result<QualityProfile> {
        let req = Request::new(path_join(&[BP_QUALITY_PROFILE, &str_val(profile.id)]))
            .with_json(profile)?;
        self.api.put_into(req).await
    }

    /// Deletes a quality profile.
    pub async fn delete_quality_profile(&self, profile_id: i64) -> Result<()> {
        self.api
            .delete_any(Request::new(path_join(&[
                BP_QUALITY_PROFILE,
                &str_val(profile_id),
            ])))
            .await
    }
}
