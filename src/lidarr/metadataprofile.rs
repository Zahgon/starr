use super::Lidarr;
use crate::error::Result;
use crate::helpers::str_val;
use crate::interface::ApiClientExt;
use crate::req::{Request, path_join};
use crate::shared::Value;
use serde::{Deserialize, Serialize};

const BP_METADATA_PROFILE: &str = "v1/metadataprofile";

/// MetadataProfile is the `/api/v1/metadataprofile` endpoint.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MetadataProfile {
    /// Profile name.
    #[serde(default)]
    pub name: String,
    /// Profile ID.
    #[serde(default)]
    pub id: i64,
    /// Primary album types and whether they are allowed.
    #[serde(default)]
    pub primary_album_types: Vec<AlbumType>,
    /// Secondary album types and whether they are allowed.
    #[serde(default)]
    pub secondary_album_types: Vec<AlbumType>,
    /// Release statuses and whether they are allowed.
    #[serde(default)]
    pub release_statuses: Vec<ReleaseStatus>,
}

/// AlbumType is part of [`MetadataProfile`].
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AlbumType {
    /// The album type.
    #[serde(default)]
    pub album_type: Option<Value>,
    /// Whether this album type is allowed.
    #[serde(default)]
    pub allowed: bool,
}

/// ReleaseStatus is part of [`MetadataProfile`].
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ReleaseStatus {
    /// The release status.
    #[serde(default)]
    pub release_status: Option<Value>,
    /// Whether this release status is allowed.
    #[serde(default)]
    pub allowed: bool,
}

impl Lidarr {
    /// Returns the metadata profiles.
    pub async fn get_metadata_profiles(&self) -> Result<Vec<MetadataProfile>> {
        self.api.get_into(Request::new(BP_METADATA_PROFILE)).await
    }

    /// Returns a single metadata profile.
    pub async fn get_metadata_profile(&self, profile_id: i64) -> Result<MetadataProfile> {
        self.api
            .get_into(Request::new(path_join(&[
                BP_METADATA_PROFILE,
                &str_val(profile_id),
            ])))
            .await
    }

    /// Creates a metadata profile.
    pub async fn add_metadata_profile(&self, profile: &MetadataProfile) -> Result<MetadataProfile> {
        let mut profile = profile.clone();
        profile.id = 0;

        self.api
            .post_into(Request::new(BP_METADATA_PROFILE).with_json(&profile)?)
            .await
    }

    /// Updates a metadata profile.
    pub async fn update_metadata_profile(
        &self,
        profile: &MetadataProfile,
    ) -> Result<MetadataProfile> {
        let req = Request::new(path_join(&[BP_METADATA_PROFILE, &str_val(profile.id)]))
            .with_json(profile)?;
        self.api.put_into(req).await
    }

    /// Deletes a metadata profile.
    pub async fn delete_metadata_profile(&self, profile_id: i64) -> Result<()> {
        self.api
            .delete_any(Request::new(path_join(&[
                BP_METADATA_PROFILE,
                &str_val(profile_id),
            ])))
            .await
    }
}
