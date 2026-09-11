use super::Sonarr;
use crate::error::Result;
use crate::helpers::str_val;
use crate::interface::ApiClientExt;
use crate::is_default;
use crate::req::{Request, path_join};
use crate::shared::Path;
use serde::{Deserialize, Serialize};

const BP_ROOT_FOLDER: &str = "v3/rootFolder";

/// RootFolder is the `/api/v3/rootfolder` endpoint.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RootFolder {
    /// Whether the folder is reachable.
    #[serde(default, skip_serializing_if = "is_default")]
    pub accessible: bool,
    /// Root folder ID.
    #[serde(default, skip_serializing_if = "is_default")]
    pub id: i64,
    /// Bytes available on the volume.
    #[serde(default, skip_serializing_if = "is_default")]
    pub free_space: i64,
    /// Path to the root folder.
    #[serde(default)]
    pub path: String,
    /// Folders inside this root folder that are not in the library.
    #[serde(default, skip_serializing_if = "is_default")]
    pub unmapped_folders: Vec<Path>,
}

impl Sonarr {
    /// Returns all configured root folders.
    pub async fn get_root_folders(&self) -> Result<Vec<RootFolder>> {
        self.api.get_into(Request::new(BP_ROOT_FOLDER)).await
    }

    /// Returns a single root folder.
    pub async fn get_root_folder(&self, folder_id: i64) -> Result<RootFolder> {
        self.api
            .get_into(Request::new(path_join(&[
                BP_ROOT_FOLDER,
                &str_val(folder_id),
            ])))
            .await
    }

    /// Creates a root folder.
    pub async fn add_root_folder(&self, folder: &RootFolder) -> Result<RootFolder> {
        self.api
            .post_into(Request::new(BP_ROOT_FOLDER).with_json(folder)?)
            .await
    }

    /// Removes a single root folder.
    pub async fn delete_root_folder(&self, folder_id: i64) -> Result<()> {
        self.api
            .delete_any(Request::new(path_join(&[
                BP_ROOT_FOLDER,
                &str_val(folder_id),
            ])))
            .await
    }
}
