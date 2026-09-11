use super::Sonarr;
use crate::error::Result;
use crate::interface::ApiClientExt;
use crate::is_default;
use crate::req::Request;
use serde::{Deserialize, Serialize};

const BP_MEDIA_MANAGEMENT: &str = "v3/config/mediaManagement";

/// MediaManagement represents the `/config/mediaManagement` endpoint.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MediaManagement {
    /// Whether a custom script runs on import.
    #[serde(default, skip_serializing_if = "is_default")]
    pub use_script_import: bool,
    /// Whether previously downloaded episodes are unmonitored.
    #[serde(default, skip_serializing_if = "is_default")]
    pub auto_unmonitor_previously_downloaded_episodes: bool,
    /// Whether hardlinks are used instead of copies.
    #[serde(default, skip_serializing_if = "is_default")]
    pub copy_using_hardlinks: bool,
    /// Whether empty series folders are created.
    #[serde(default, skip_serializing_if = "is_default")]
    pub create_empty_series_folders: bool,
    /// Whether empty folders are deleted.
    #[serde(default, skip_serializing_if = "is_default")]
    pub delete_empty_folders: bool,
    /// Whether media info is read from files.
    #[serde(default, skip_serializing_if = "is_default")]
    pub enable_media_info: bool,
    /// Whether extra files are imported.
    #[serde(default, skip_serializing_if = "is_default")]
    pub import_extra_files: bool,
    /// Whether permissions are set on Linux.
    #[serde(default, skip_serializing_if = "is_default")]
    pub set_permissions_linux: bool,
    /// Whether the free space check is skipped when importing.
    #[serde(default, skip_serializing_if = "is_default")]
    pub skip_free_space_check_when_importing: bool,
    /// Config ID.
    #[serde(default)]
    pub id: i64,
    /// Free space required to import, in bytes. Zero or empty is not allowed.
    #[serde(default)]
    pub minimum_free_space_when_importing: i64,
    /// Days before the recycle bin is emptied.
    #[serde(default, skip_serializing_if = "is_default")]
    pub recycle_bin_cleanup_days: i64,
    /// Path to the import script.
    #[serde(default, skip_serializing_if = "is_default")]
    pub script_import_path: String,
    /// Permissions applied to folders.
    #[serde(default, skip_serializing_if = "is_default")]
    pub chmod_folder: String,
    /// Group applied to files. An empty string is valid.
    #[serde(default)]
    pub chown_group: String,
    /// When propers and repacks are downloaded.
    #[serde(default, skip_serializing_if = "is_default")]
    pub download_propers_and_repacks: String,
    /// When an episode title is required to import.
    #[serde(default, skip_serializing_if = "is_default")]
    pub episode_title_required: String,
    /// Extra file extensions to import.
    #[serde(default, skip_serializing_if = "is_default")]
    pub extra_file_extensions: String,
    /// Which date is written to imported files.
    #[serde(default, skip_serializing_if = "is_default")]
    pub file_date: String,
    /// Path to the recycle bin. An empty string is valid.
    #[serde(default)]
    pub recycle_bin: String,
    /// When the library is rescanned after a refresh.
    #[serde(default, skip_serializing_if = "is_default")]
    pub rescan_after_refresh: String,
}

impl Sonarr {
    /// Returns the media management config.
    pub async fn get_media_management(&self) -> Result<MediaManagement> {
        self.api.get_into(Request::new(BP_MEDIA_MANAGEMENT)).await
    }

    /// Updates the media management config.
    pub async fn update_media_management(
        &self,
        media_management: &MediaManagement,
    ) -> Result<MediaManagement> {
        let req = Request::new(BP_MEDIA_MANAGEMENT).with_json(media_management)?;
        self.api.put_into(req).await
    }
}
