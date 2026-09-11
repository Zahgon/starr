use super::Readarr;
use crate::error::Result;
use crate::interface::ApiClientExt;
use crate::req::Request;
use serde::{Deserialize, Serialize};

const BP_MEDIA_MANAGEMENT: &str = "v1/config/mediaManagement";

/// MediaManagement represents the `/config/mediaManagement` endpoint.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MediaManagement {
    /// Whether the free space check is skipped when importing.
    #[serde(default)]
    pub skip_free_space_check_when_importing: bool,
    /// Whether previously downloaded books are unmonitored.
    #[serde(default)]
    pub auto_unmonitor_previously_downloaded_books: bool,
    /// Whether permissions are set on Linux.
    #[serde(default)]
    pub set_permissions_linux: bool,
    /// Whether empty author folders are created.
    #[serde(default)]
    pub create_empty_author_folders: bool,
    /// Whether empty folders are deleted.
    #[serde(default)]
    pub delete_empty_folders: bool,
    /// Whether the library is watched for changes.
    #[serde(default)]
    pub watch_library_for_changes: bool,
    /// Whether hardlinks are used instead of copies.
    #[serde(default)]
    pub copy_using_hardlinks: bool,
    /// Whether extra files are imported.
    #[serde(default)]
    pub import_extra_files: bool,
    /// Free space required to import, in bytes.
    #[serde(default)]
    pub minimum_free_space_when_importing: i64,
    /// Config ID. Always 1.
    #[serde(default)]
    pub id: i64,
    /// Days before the recycle bin is emptied.
    #[serde(default)]
    pub recycle_bin_cleanup_days: i64,
    /// Path to the recycle bin.
    #[serde(default)]
    pub recycle_bin: String,
    /// When propers and repacks are downloaded.
    #[serde(default)]
    pub download_propers_and_repacks: String,
    /// Which date is written to imported files.
    #[serde(default)]
    pub file_date: String,
    /// When the library is rescanned after a refresh.
    #[serde(default)]
    pub rescan_after_refresh: String,
    /// When fingerprinting is used to match files.
    #[serde(default)]
    pub allow_fingerprinting: String,
    /// Permissions applied to folders.
    #[serde(default)]
    pub chmod_folder: String,
    /// Group applied to files.
    #[serde(default)]
    pub chown_group: String,
    /// Extra file extensions to import.
    #[serde(default)]
    pub extra_file_extensions: String,
}

impl Readarr {
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
