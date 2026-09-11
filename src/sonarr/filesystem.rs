use super::Sonarr;
use crate::error::{Error, Result};
use crate::helpers::str_val;
use crate::interface::ApiClientExt;
use crate::req::{Request, path_join, set_api_path};
use crate::shared::Path;
use crate::values::Values;

const BP_FILESYSTEM: &str = "v3/filesystem";

/// FilesystemQuery is the query for `/api/v3/filesystem`.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct FilesystemQuery {
    /// Path to browse.
    pub path: String,
    /// Whether files are included in the listing.
    pub include_files: bool,
    /// Whether folders without trailing slashes are allowed.
    pub allow_folders_without_trailing_slashes: bool,
}

impl FilesystemQuery {
    /// Builds query parameters for the filesystem browser.
    pub fn values(&self) -> Values {
        let mut val = Values::new();

        if !self.path.is_empty() {
            val.set("path", self.path.clone());
        }

        val.set("includeFiles", str_val(self.include_files));
        val.set(
            "allowFoldersWithoutTrailingSlashes",
            str_val(self.allow_folders_without_trailing_slashes),
        );

        val
    }
}

impl Sonarr {
    /// Lists files and folders for a path.
    pub async fn browse_filesystem(&self, query: Option<&FilesystemQuery>) -> Result<Vec<Path>> {
        let query = query.map(FilesystemQuery::values).unwrap_or_default();

        self.api
            .get_into(Request::new(BP_FILESYSTEM).with_query(query))
            .await
    }

    /// Lists media files under a path.
    pub async fn browse_filesystem_media_files(&self, path_name: &str) -> Result<Vec<Path>> {
        let mut params = Values::new();
        if !path_name.is_empty() {
            params.set("path", path_name);
        }

        self.api
            .get_into(Request::new(path_join(&[BP_FILESYSTEM, "mediafiles"])).with_query(params))
            .await
    }

    /// Returns the raw response body from `/api/v3/filesystem/type`.
    pub async fn get_filesystem_type(&self, path_name: &str) -> Result<Vec<u8>> {
        let mut params = Values::new();
        params.set("path", path_name);

        // Go builds this raw path without the /api prefix, which yields a
        // malformed URL; the prefix is added here so the call works.
        let uri = set_api_path(&path_join(&[BP_FILESYSTEM, "type"]));

        let resp = self
            .api
            .get(Request::new(uri).with_query(params))
            .await?;

        let body = resp.bytes().await.map_err(|source| Error::Http {
            context: "reading HTTP response body".to_string(),
            source,
        })?;

        Ok(body.to_vec())
    }
}
