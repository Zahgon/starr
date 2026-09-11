use super::Radarr;
use crate::error::Result;
use crate::helpers::str_val;
use crate::interface::ApiClientExt;
use crate::is_default;
use crate::req::{Request, path_join};
use serde::{Deserialize, Serialize};

const BP_RELEASE_PROFILE: &str = "v3/releaseProfile";

/// ReleaseProfile defines a release profile's data from Radarr. v4 only.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ReleaseProfile {
    /// Profile name.
    #[serde(default)]
    pub name: String,
    /// Whether the profile is enabled.
    #[serde(default)]
    pub enabled: bool,
    /// Terms a release must contain.
    #[serde(default)]
    pub required: Vec<String>,
    /// Terms a release must not contain.
    #[serde(default)]
    pub ignored: Vec<String>,
    /// Indexer this profile applies to.
    #[serde(default, rename = "indexerId")]
    pub indexer_id: i64,
    /// Tags this profile applies to.
    #[serde(default)]
    pub tags: Vec<i32>,
    /// Profile ID.
    #[serde(default, skip_serializing_if = "is_default")]
    pub id: i64,
}

impl Radarr {
    /// Returns all configured release profiles.
    pub async fn get_release_profiles(&self) -> Result<Vec<ReleaseProfile>> {
        self.api.get_into(Request::new(BP_RELEASE_PROFILE)).await
    }

    /// Returns a single release profile.
    pub async fn get_release_profile(&self, profile_id: i64) -> Result<ReleaseProfile> {
        self.api
            .get_into(Request::new(path_join(&[
                BP_RELEASE_PROFILE,
                &str_val(profile_id),
            ])))
            .await
    }

    /// Creates a release profile.
    pub async fn add_release_profile(&self, profile: &ReleaseProfile) -> Result<ReleaseProfile> {
        self.api
            .post_into(Request::new(BP_RELEASE_PROFILE).with_json(profile)?)
            .await
    }

    /// Updates the release profile.
    pub async fn update_release_profile(&self, profile: &ReleaseProfile) -> Result<ReleaseProfile> {
        let req = Request::new(path_join(&[BP_RELEASE_PROFILE, &str_val(profile.id)]))
            .with_json(profile)?;
        self.api.put_into(req).await
    }

    /// Removes a single release profile.
    pub async fn delete_release_profile(&self, profile_id: i64) -> Result<()> {
        self.api
            .delete_any(Request::new(path_join(&[
                BP_RELEASE_PROFILE,
                &str_val(profile_id),
            ])))
            .await
    }
}
