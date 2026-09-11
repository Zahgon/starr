use super::Readarr;
use crate::error::Result;
use crate::interface::ApiClientExt;
use crate::is_default;
use crate::req::Request;
use serde::{Deserialize, Serialize};

const BP_METADATA_PROFILE: &str = "v1/metadataProfile";

/// MetadataProfile is the `/api/v1/metadataProfile` endpoint.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MetadataProfile {
    /// Profile ID.
    #[serde(default)]
    pub id: i64,
    /// Profile name.
    #[serde(default)]
    pub name: String,
    /// Minimum popularity a book needs to be included.
    #[serde(default)]
    pub min_popularity: f64,
    /// Whether books without a release date are skipped.
    #[serde(default)]
    pub skip_missing_date: bool,
    /// Whether books without an ISBN are skipped.
    #[serde(default, rename = "skipMissingIsbn")]
    pub skip_missing_isbn: bool,
    /// Whether parts and sets are skipped.
    #[serde(default)]
    pub skip_parts_and_sets: bool,
    /// Whether secondary series entries are skipped.
    #[serde(default)]
    pub skip_series_secondary: bool,
    /// Languages allowed by this profile.
    #[serde(default, skip_serializing_if = "is_default")]
    pub allowed_languages: String,
}

impl Readarr {
    /// Returns the metadata profiles.
    pub async fn get_metadata_profiles(&self) -> Result<Vec<MetadataProfile>> {
        self.api.get_into(Request::new(BP_METADATA_PROFILE)).await
    }
}
