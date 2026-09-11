use super::{
    Album, Artist, AudioTags, Lidarr, ManualImportCommandRequest, ManualImportFile, Track,
};
use crate::error::Result;
use crate::helpers::str_val;
use crate::interface::ApiClientExt;
use crate::req::Request;
use crate::shared::Quality;
use crate::values::Values;
use serde::{Deserialize, Serialize};

const BP_MANUAL_IMPORT: &str = "v1/manualimport";

/// ManualImportInput is the input data for a manual import request using a POST request.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ManualImportInput {
    /// Manual import item ID.
    #[serde(default)]
    pub id: i64,
    /// Path to the file.
    #[serde(default)]
    pub path: String,
    /// File name.
    #[serde(default)]
    pub name: String,
    /// Artist the file belongs to.
    #[serde(default, rename = "artistID")]
    pub artist_id: i64,
    /// Album the file belongs to.
    #[serde(default, rename = "albumID")]
    pub album_id: i64,
    /// Album release the file belongs to.
    #[serde(default)]
    pub album_release_id: i64,
    /// Tracks the file contains.
    #[serde(default)]
    pub tracks: Vec<Track>,
    /// Track IDs the file contains.
    #[serde(default)]
    pub track_ids: Vec<i64>,
    /// Quality of the file.
    #[serde(default)]
    pub quality: Option<Quality>,
    /// Release group that produced the release.
    #[serde(default)]
    pub release_group: String,
    /// Download client ID the file came from.
    #[serde(default)]
    pub download_id: String,
    /// Whether this is an additional file.
    #[serde(default)]
    pub additional_file: bool,
    /// Whether existing files are replaced.
    #[serde(default)]
    pub replace_existing_files: bool,
    /// Whether release switching is disabled.
    #[serde(default)]
    pub disable_release_switching: bool,
    /// Why the file cannot be imported.
    #[serde(default)]
    pub rejections: Vec<Rejection>,
}

/// ManualImportOutput is the output data for a manual import request.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ManualImportOutput {
    /// Manual import item ID.
    #[serde(default)]
    pub id: i64,
    /// Path to the file.
    #[serde(default)]
    pub path: String,
    /// File name.
    #[serde(default)]
    pub name: String,
    /// File size in bytes.
    #[serde(default)]
    pub size: i32,
    /// Artist the file belongs to.
    #[serde(default)]
    pub artist: Option<Artist>,
    /// Album the file belongs to.
    #[serde(default)]
    pub album: Option<Album>,
    /// Album release the file belongs to.
    #[serde(default)]
    pub album_release_id: i64,
    /// Tracks the file contains.
    #[serde(default)]
    pub tracks: Vec<Track>,
    /// Quality of the file.
    #[serde(default)]
    pub quality: Option<Quality>,
    /// Release group that produced the release.
    #[serde(default)]
    pub release_group: String,
    /// Weight of the quality.
    #[serde(default)]
    pub quality_weight: i64,
    /// Download client ID the file came from.
    #[serde(default)]
    pub download_id: String,
    /// Audio tags read from the file.
    #[serde(default)]
    pub audio_tags: Option<AudioTags>,
    /// Whether this is an additional file.
    #[serde(default)]
    pub additional_file: bool,
    /// Whether existing files are replaced.
    #[serde(default)]
    pub replace_existing_files: bool,
    /// Whether release switching is disabled.
    #[serde(default)]
    pub disable_release_switching: bool,
    /// Why the file cannot be imported.
    #[serde(default)]
    pub rejections: Vec<Rejection>,
}

/// Rejection is part of the manual import payload.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Rejection {
    /// Why the file was rejected.
    #[serde(default)]
    pub reason: String,
    /// Whether the rejection is `permanent` or `temporary`.
    #[serde(default, rename = "type")]
    pub rejection_type: String,
}

/// ManualImportParams provides the input parameters for the `GET /manualimport` API.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct ManualImportParams {
    /// Folder to scan for importable files.
    pub folder: String,
    /// Download client ID to scan.
    pub download_id: String,
    /// Artist the files belong to.
    pub artist_id: i64,
    /// Whether existing files are replaced.
    pub replace_existing_files: bool,
    /// Whether files already in the library are filtered out.
    pub filter_existing_files: bool,
}

/// Builds a [`ManualImportCommandRequest`] from the `GET manualimport` response.
///
/// Use with [`Lidarr::send_manual_import_command`] to trigger import of the
/// listed files (e.g. after a FLAC+CUE split). Returns `None` when there is
/// nothing to import.
pub fn manual_import_command_from_outputs(
    outputs: &[ManualImportOutput],
    replace_existing: bool,
) -> Option<ManualImportCommandRequest> {
    if outputs.is_empty() {
        return None;
    }

    let files: Vec<ManualImportFile> = outputs
        .iter()
        .map(|output| ManualImportFile {
            path: output.path.clone(),
            artist_id: output.artist.as_ref().map_or(0, |artist| artist.id),
            album_id: output.album.as_ref().map_or(0, |album| album.id),
            album_release_id: output.album_release_id,
            track_ids: output.tracks.iter().map(|track| track.id).collect(),
            quality: Some(output.quality.clone().unwrap_or_default()),
            indexer_flags: 0,
            download_id: output.download_id.clone(),
            disable_release_switching: output.disable_release_switching,
        })
        .collect();

    if files.is_empty() {
        return None;
    }

    Some(ManualImportCommandRequest {
        name: "ManualImport".to_string(),
        files,
        import_mode: "auto".to_string(),
        replace_existing_files: replace_existing,
    })
}

impl Lidarr {
    /// Returns the list of files available for manual import in the given folder.
    pub async fn manual_import(
        &self,
        params: &ManualImportParams,
    ) -> Result<Vec<ManualImportOutput>> {
        let mut query = Values::new();
        query.add("folder", params.folder.clone());
        query.add("downloadId", params.download_id.clone());
        query.add("artistId", str_val(params.artist_id));
        query.add("replaceExistingFiles", str_val(params.replace_existing_files));
        query.add("filterExistingFiles", str_val(params.filter_existing_files));

        self.api
            .get_into(Request::new(BP_MANUAL_IMPORT).with_query(query))
            .await
    }

    /// Reprocesses a manual import.
    pub async fn manual_import_reprocess(&self, manualimport: &ManualImportInput) -> Result<()> {
        let req = Request::new(BP_MANUAL_IMPORT).with_json(manualimport)?;
        self.api.post_any(req).await
    }
}
