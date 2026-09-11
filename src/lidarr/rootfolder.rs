use super::Lidarr;
use crate::error::Result;
use crate::interface::ApiClientExt;
use crate::req::Request;
use crate::shared::Path;
use serde::{Deserialize, Serialize};

const BP_ROOT_FOLDER: &str = "v1/rootFolder";

/// RootFolder is the `/api/v1/rootfolder` endpoint.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RootFolder {
    /// Root folder ID.
    #[serde(default)]
    pub id: i64,
    /// Path to the root folder.
    #[serde(default)]
    pub path: String,
    /// Free space in bytes.
    #[serde(default)]
    pub free_space: i64,
    /// Total space in bytes.
    #[serde(default)]
    pub total_space: i64,
    /// Folders inside the root folder that are not mapped to an artist.
    #[serde(default)]
    pub unmapped_folders: Vec<Path>,
}

impl Lidarr {
    /// Returns all configured root folders.
    pub async fn get_root_folders(&self) -> Result<Vec<RootFolder>> {
        self.api.get_into(Request::new(BP_ROOT_FOLDER)).await
    }
}
