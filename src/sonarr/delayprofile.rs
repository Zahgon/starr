use super::Sonarr;
use crate::error::Result;
use crate::helpers::str_val;
use crate::interface::ApiClientExt;
use crate::is_default;
use crate::req::{Request, path_join};
use crate::shared::Protocol;
use serde::{Deserialize, Serialize};

const BP_DELAY_PROFILE: &str = "v3/delayProfile";

/// DelayProfile is the `/api/v3/delayprofile` endpoint.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DelayProfile {
    /// Whether usenet releases are delayed.
    #[serde(default, skip_serializing_if = "is_default")]
    pub enable_usenet: bool,
    /// Whether torrent releases are delayed.
    #[serde(default, skip_serializing_if = "is_default")]
    pub enable_torrent: bool,
    /// Whether the delay is skipped for the highest quality.
    #[serde(default, skip_serializing_if = "is_default")]
    pub bypass_if_highest_quality: bool,
    /// Minutes to delay usenet releases.
    #[serde(default, skip_serializing_if = "is_default")]
    pub usenet_delay: i64,
    /// Minutes to delay torrent releases.
    #[serde(default, skip_serializing_if = "is_default")]
    pub torrent_delay: i64,
    /// Profile ID.
    #[serde(default, skip_serializing_if = "is_default")]
    pub id: i64,
    /// Order this profile is applied in.
    #[serde(default, skip_serializing_if = "is_default")]
    pub order: i64,
    /// Tags this profile applies to.
    #[serde(default)]
    pub tags: Vec<i32>,
    /// Protocol preferred when both are available.
    #[serde(default, skip_serializing_if = "is_default")]
    pub preferred_protocol: Protocol,
}

impl Sonarr {
    /// Returns all configured delay profiles.
    pub async fn get_delay_profiles(&self) -> Result<Vec<DelayProfile>> {
        self.api.get_into(Request::new(BP_DELAY_PROFILE)).await
    }

    /// Returns a single delay profile.
    pub async fn get_delay_profile(&self, profile_id: i64) -> Result<DelayProfile> {
        self.api
            .get_into(Request::new(path_join(&[
                BP_DELAY_PROFILE,
                &str_val(profile_id),
            ])))
            .await
    }

    /// Creates a delay profile.
    pub async fn add_delay_profile(&self, profile: &DelayProfile) -> Result<DelayProfile> {
        self.api
            .post_into(Request::new(BP_DELAY_PROFILE).with_json(profile)?)
            .await
    }

    /// Updates the delay profile.
    pub async fn update_delay_profile(&self, profile: &DelayProfile) -> Result<DelayProfile> {
        let req =
            Request::new(path_join(&[BP_DELAY_PROFILE, &str_val(profile.id)])).with_json(profile)?;
        self.api.put_into(req).await
    }

    /// Removes a single delay profile.
    pub async fn delete_delay_profile(&self, profile_id: i64) -> Result<()> {
        self.api
            .delete_any(Request::new(path_join(&[
                BP_DELAY_PROFILE,
                &str_val(profile_id),
            ])))
            .await
    }
}
