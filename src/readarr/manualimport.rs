use super::{AudioTags, Author, Book, Readarr};
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
    /// Author the file belongs to.
    #[serde(default, rename = "authorID")]
    pub author_id: i64,
    /// Book the file belongs to.
    #[serde(default, rename = "bookID")]
    pub book_id: i64,
    /// Edition the file belongs to.
    #[serde(default)]
    pub foreign_edition_id: i64,
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
    /// The author the file belongs to.
    #[serde(default)]
    pub author: Option<Author>,
    /// The book the file belongs to.
    #[serde(default)]
    pub book: Option<Book>,
    /// Edition the file belongs to.
    #[serde(default)]
    pub foreign_edition_id: i64,
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
    /// Author the files belong to.
    pub author_id: i64,
    /// Whether existing files are replaced.
    pub replace_existing_files: bool,
    /// Whether files already in the library are filtered out.
    pub filter_existing_files: bool,
}

impl Readarr {
    /// Initiates a manual import.
    pub async fn manual_import(&self, params: &ManualImportParams) -> Result<ManualImportOutput> {
        let mut query = Values::new();
        query.add("folder", params.folder.clone());
        query.add("downloadId", params.download_id.clone());
        query.add("authorId", str_val(params.author_id));
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
