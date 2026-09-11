use super::Readarr;
use crate::error::Result;
use crate::interface::ApiClientExt;
use crate::req::Request;
use serde::{Deserialize, Serialize};

const BP_ROOT_FOLDER: &str = "v1/rootFolder";

/// RootFolder is the `/api/v1/rootfolder` endpoint.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RootFolder {
    /// Root folder ID.
    #[serde(default)]
    pub id: i64,
    /// Root folder name.
    #[serde(default)]
    pub name: String,
    /// Path to the root folder.
    #[serde(default)]
    pub path: String,
    /// Metadata profile applied to authors added here.
    #[serde(default)]
    pub default_metadata_profile_id: i64,
    /// Quality profile applied to authors added here.
    #[serde(default)]
    pub default_quality_profile_id: i64,
    /// What gets monitored on authors added here.
    #[serde(default)]
    pub default_monitor_option: String,
    /// Tags applied to authors added here.
    #[serde(default)]
    pub default_tags: Vec<i32>,
    /// Calibre server port.
    #[serde(default)]
    pub port: i32,
    /// Calibre output profile.
    #[serde(default)]
    pub output_profile: String,
    /// Whether the Calibre server uses SSL.
    #[serde(default, rename = "useSsl")]
    pub use_ssl: bool,
    /// Whether the folder is reachable.
    #[serde(default)]
    pub accessible: bool,
    /// Whether this folder is a Calibre library.
    #[serde(default)]
    pub is_calibre_library: bool,
    /// Bytes available on the volume.
    #[serde(default)]
    pub free_space: i64,
    /// Total bytes on the volume.
    #[serde(default)]
    pub total_space: i64,
}

impl Readarr {
    /// Returns all configured root folders.
    pub async fn get_root_folders(&self) -> Result<Vec<RootFolder>> {
        self.api.get_into(Request::new(BP_ROOT_FOLDER)).await
    }
}
