use super::{CustomFormatInput, CustomFormatOutput, Episode, Series, Sonarr};
use crate::error::Result;
use crate::helpers::str_val;
use crate::interface::ApiClientExt;
use crate::req::Request;
use crate::shared::{Quality, Value};
use crate::values::Values;
use serde::{Deserialize, Serialize};

const BP_MANUAL_IMPORT: &str = "v3/manualimport";

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
    /// Series the file belongs to.
    #[serde(default)]
    pub series_id: i64,
    /// Season the file belongs to.
    #[serde(default)]
    pub season_number: i32,
    /// Episodes the file covers.
    #[serde(default)]
    pub episodes: Vec<Episode>,
    /// Episode IDs the file covers.
    #[serde(default)]
    pub episode_ids: Vec<i64>,
    /// Quality of the file.
    #[serde(default)]
    pub quality: Option<Quality>,
    /// Languages found in the file.
    #[serde(default)]
    pub languages: Vec<Value>,
    /// Release group that produced the release.
    #[serde(default)]
    pub release_group: String,
    /// Download client ID the file came from.
    #[serde(default)]
    pub download_id: String,
    /// Custom formats matched by the file.
    #[serde(default)]
    pub custom_formats: Vec<CustomFormatInput>,
    /// Total score of the matched custom formats.
    #[serde(default)]
    pub custom_format_score: i64,
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
    /// Path relative to the series folder.
    #[serde(default)]
    pub relative_path: String,
    /// Name of the containing folder.
    #[serde(default)]
    pub folder_name: String,
    /// File name.
    #[serde(default)]
    pub name: String,
    /// File size in bytes.
    #[serde(default)]
    pub size: i64,
    /// The series the file belongs to.
    #[serde(default)]
    pub series: Option<Series>,
    /// Season the file belongs to.
    #[serde(default)]
    pub season_number: i32,
    /// Episodes the file covers.
    #[serde(default)]
    pub episodes: Vec<Episode>,
    /// Episode file backing these episodes.
    #[serde(default)]
    pub episode_file_id: i64,
    /// Quality of the file.
    #[serde(default)]
    pub quality: Option<Quality>,
    /// Languages found in the file.
    #[serde(default)]
    pub languages: Vec<Value>,
    /// Release group that produced the release.
    #[serde(default)]
    pub release_group: String,
    /// Weight of the quality.
    #[serde(default)]
    pub quality_weight: i64,
    /// Download client ID the file came from.
    #[serde(default)]
    pub download_id: String,
    /// Custom formats matched by the file.
    #[serde(default)]
    pub custom_formats: Vec<CustomFormatOutput>,
    /// Total score of the matched custom formats.
    #[serde(default)]
    pub custom_format_score: i64,
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
    /// Series the files belong to.
    pub series_id: i64,
    /// Season the files belong to.
    pub season_number: i32,
    /// Whether files already in the library are filtered out.
    pub filter_existing_files: bool,
}

impl Sonarr {
    /// Initiates a manual import.
    pub async fn manual_import(&self, params: &ManualImportParams) -> Result<ManualImportOutput> {
        let mut query = Values::new();
        query.add("folder", params.folder.clone());
        query.add("downloadId", params.download_id.clone());
        query.add("seriesId", str_val(params.series_id));
        query.add("seasonNumber", str_val(params.season_number));
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
