use super::Lidarr;
use crate::error::Result;
use crate::interface::ApiClientExt;
use crate::req::Request;
use serde::{Deserialize, Serialize};

const BP_MEDIA_MANAGEMENT: &str = "v1/config/mediaManagement";

/// MediaManagement represents the `/config/mediamanagement` endpoint.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MediaManagement {
    /// Whether the free space check is skipped when importing.
    #[serde(default)]
    pub skip_free_space_check_when_importing: bool,
    /// Whether hardlinks are used instead of copies.
    #[serde(default)]
    pub copy_using_hardlinks: bool,
    /// Whether extra files are imported alongside tracks.
    #[serde(default)]
    pub import_extra_files: bool,
    /// Whether the library is watched for changes.
    #[serde(default)]
    pub watch_library_for_changes: bool,
    /// Whether previously downloaded tracks are unmonitored automatically.
    #[serde(default)]
    pub auto_unmonitor_previously_downloaded_tracks: bool,
    /// Whether empty artist folders are created.
    #[serde(default)]
    pub create_empty_artist_folders: bool,
    /// Whether empty folders are deleted.
    #[serde(default)]
    pub delete_empty_folders: bool,
    /// Whether permissions are set on Linux.
    #[serde(default)]
    pub set_permissions_linux: bool,
    /// Minimum free space in megabytes required to import.
    #[serde(default)]
    pub minimum_free_space_when_importing: i64,
    /// How long items stay in the recycle bin, in days.
    #[serde(default)]
    pub recycle_bin_cleanup_days: i64,
    /// Config ID. Always 1.
    #[serde(default)]
    pub id: i64,
    /// Path to the recycle bin.
    #[serde(default)]
    pub recycle_bin: String,
    /// Whether propers and repacks are downloaded.
    #[serde(default)]
    pub download_propers_and_repacks: String,
    /// Which date is written to imported files.
    #[serde(default)]
    pub file_date: String,
    /// When the library is rescanned after a refresh.
    #[serde(default)]
    pub rescan_after_refresh: String,
    /// Whether fingerprinting is allowed.
    #[serde(default)]
    pub allow_fingerprinting: String,
    /// Folder permissions in octal.
    #[serde(default)]
    pub chmod_folder: String,
    /// Group applied to new folders.
    #[serde(default)]
    pub chown_group: String,
    /// Extra file extensions imported alongside tracks.
    #[serde(default)]
    pub extra_file_extensions: String,
}

impl Lidarr {
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
