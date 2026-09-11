use super::Prowlarr;
use crate::error::Result;
use crate::helpers::str_val;
use crate::interface::ApiClientExt;
use crate::is_default;
use crate::req::{Request, path_join};
use serde::{Deserialize, Serialize};

const BP_APP_PROFILE: &str = "v1/appprofile";

/// AppProfile is a Prowlarr application profile.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AppProfile {
    /// Profile ID. Leave zero when creating.
    #[serde(default, skip_serializing_if = "is_default")]
    pub id: i64,
    /// Profile name.
    #[serde(default, skip_serializing_if = "is_default")]
    pub name: String,
    /// Whether RSS sync is enabled for indexers using this profile.
    #[serde(default)]
    pub enable_rss: bool,
    /// Whether automatic search is enabled.
    #[serde(default)]
    pub enable_automatic_search: bool,
    /// Whether interactive search is enabled.
    #[serde(default)]
    pub enable_interactive_search: bool,
    /// Minimum seeders required for a torrent release.
    #[serde(default, skip_serializing_if = "is_default")]
    pub minimum_seeders: i32,
}

impl Prowlarr {
    /// Returns all application profiles.
    pub async fn get_app_profiles(&self) -> Result<Vec<AppProfile>> {
        self.api.get_into(Request::new(BP_APP_PROFILE)).await
    }

    /// Returns a single application profile.
    pub async fn get_app_profile(&self, id: i64) -> Result<AppProfile> {
        self.api
            .get_into(Request::new(path_join(&[BP_APP_PROFILE, &str_val(id)])))
            .await
    }

    /// Returns default application profile templates.
    pub async fn get_app_profile_schema(&self) -> Result<AppProfile> {
        self.api
            .get_into(Request::new(path_join(&[BP_APP_PROFILE, "schema"])))
            .await
    }

    /// Creates an application profile.
    pub async fn add_app_profile(&self, profile: &AppProfile) -> Result<AppProfile> {
        self.api
            .post_into(Request::new(BP_APP_PROFILE).with_json(profile)?)
            .await
    }

    /// Updates an application profile.
    pub async fn update_app_profile(&self, profile: &AppProfile) -> Result<AppProfile> {
        let req =
            Request::new(path_join(&[BP_APP_PROFILE, &str_val(profile.id)])).with_json(profile)?;
        self.api.put_into(req).await
    }

    /// Removes an application profile.
    pub async fn delete_app_profile(&self, id: i64) -> Result<()> {
        self.api
            .delete_any(Request::new(path_join(&[BP_APP_PROFILE, &str_val(id)])))
            .await
    }
}
