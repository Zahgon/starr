use super::Sonarr;
use crate::error::{Error, Result};
use crate::helpers::str_val;
use crate::interface::ApiClientExt;
use crate::req::{Request, multipart_form_file, path_join, set_api_path};
use crate::shared::BackupFile;
use chrono::{DateTime, Utc};
use reqwest::header::{CONTENT_TYPE, HeaderMap, HeaderValue};
use serde::{Deserialize, Serialize};

const BP_SYSTEM: &str = "v3/system";

/// SystemTask is a scheduled task from `/api/v3/system/task`.
pub type SystemTask = crate::starrshared::SystemTask;

/// BackupRestoreResponse is returned when restoring a backup.
pub type BackupRestoreResponse = crate::starrshared::BackupRestoreResponse;

/// SystemStatus is the `/api/v3/system/status` endpoint.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SystemStatus {
    /// Path to the application data folder.
    #[serde(default)]
    pub app_data: String,
    /// Application name.
    #[serde(default)]
    pub app_name: String,
    /// Authentication method in use.
    #[serde(default)]
    pub authentication: String,
    /// Branch the app updates from.
    #[serde(default)]
    pub branch: String,
    /// When the running build was produced.
    #[serde(default)]
    pub build_time: Option<DateTime<Utc>>,
    /// Name given to this instance.
    #[serde(default)]
    pub instance_name: String,
    /// Whether the app runs with admin rights.
    #[serde(default)]
    pub is_admin: bool,
    /// Whether debug mode is on.
    #[serde(default)]
    pub is_debug: bool,
    /// Whether the host is Linux.
    #[serde(default)]
    pub is_linux: bool,
    /// Whether the app runs on Mono.
    #[serde(default)]
    pub is_mono: bool,
    /// Whether the runtime is Mono.
    #[serde(default)]
    pub is_mono_runtime: bool,
    /// Whether the host is macOS.
    #[serde(default)]
    pub is_osx: bool,
    /// Whether this is a production build.
    #[serde(default)]
    pub is_production: bool,
    /// Whether the app runs interactively.
    #[serde(default)]
    pub is_user_interactive: bool,
    /// Whether the host is Windows.
    #[serde(default)]
    pub is_windows: bool,
    /// Mode the app runs in.
    #[serde(default)]
    pub mode: String,
    /// Operating system name.
    #[serde(default)]
    pub os_name: String,
    /// Version of the operating system.
    #[serde(default)]
    pub os_version: String,
    /// Who packaged this build.
    #[serde(default)]
    pub package_author: String,
    /// How this package updates itself.
    #[serde(default)]
    pub package_update_mechanism: String,
    /// Version of the package.
    #[serde(default)]
    pub package_version: String,
    /// Name of the runtime.
    #[serde(default)]
    pub runtime_name: String,
    /// Version of the runtime.
    #[serde(default)]
    pub runtime_version: String,
    /// Version of SQLite in use.
    #[serde(default)]
    pub sqlite_version: String,
    /// When the app started.
    #[serde(default)]
    pub start_time: Option<DateTime<Utc>>,
    /// Path the app started from.
    #[serde(default)]
    pub startup_path: String,
    /// URL base the app is served under.
    #[serde(default, rename = "urlBase")]
    pub url_base: String,
    /// Application version.
    #[serde(default)]
    pub version: String,
}

impl Sonarr {
    /// Returns system status.
    pub async fn get_system_status(&self) -> Result<SystemStatus> {
        self.api
            .get_into(Request::new(path_join(&[BP_SYSTEM, "status"])))
            .await
    }

    /// Returns all available Sonarr backup files.
    ///
    /// Use a raw GET to download a file using `BackupFile.path`.
    pub async fn get_backup_files(&self) -> Result<Vec<BackupFile>> {
        self.api
            .get_into(Request::new(path_join(&[BP_SYSTEM, "backup"])))
            .await
    }

    /// Deletes a backup file by ID.
    pub async fn delete_backup(&self, id: i64) -> Result<()> {
        self.api
            .delete_any(Request::new(path_join(&[
                BP_SYSTEM,
                "backup",
                &str_val(id),
            ])))
            .await
    }

    /// Restores an on-disk backup by ID.
    pub async fn restore_backup(&self, id: i64) -> Result<BackupRestoreResponse> {
        self.api
            .post_into(Request::new(path_join(&[
                BP_SYSTEM,
                "backup",
                "restore",
                &str_val(id),
            ])))
            .await
    }

    /// Uploads a backup archive and restores it.
    pub async fn restore_backup_upload(
        &self,
        filename: &str,
        file: &[u8],
    ) -> Result<BackupRestoreResponse> {
        let (body, content_type) = multipart_form_file("file", filename, file);

        let mut headers = HeaderMap::new();
        headers.insert(
            CONTENT_TYPE,
            HeaderValue::from_str(&content_type).expect("multipart content type is always valid"),
        );

        let req = Request::new(path_join(&[BP_SYSTEM, "backup", "restore", "upload"]))
            .with_body(body)
            .with_headers(headers);

        self.api.post_into(req).await
    }

    /// Tells Sonarr to restart.
    pub async fn restart(&self) -> Result<()> {
        self.api
            .post_any(Request::new(path_join(&[BP_SYSTEM, "restart"])))
            .await
    }

    /// Tells Sonarr to shut down.
    pub async fn shutdown(&self) -> Result<()> {
        self.api
            .post_any(Request::new(path_join(&[BP_SYSTEM, "shutdown"])))
            .await
    }

    /// Returns all scheduled tasks.
    pub async fn get_system_tasks(&self) -> Result<Vec<SystemTask>> {
        self.api
            .get_into(Request::new(path_join(&[BP_SYSTEM, "task"])))
            .await
    }

    /// Returns the raw JSON route table (schema-less in OpenAPI).
    pub async fn get_system_routes(&self) -> Result<Vec<u8>> {
        self.read_raw(&set_api_path(&path_join(&[BP_SYSTEM, "routes"]))).await
    }

    /// Returns duplicate route definitions as raw JSON.
    pub async fn get_system_duplicate_routes(&self) -> Result<Vec<u8>> {
        self.read_raw(&set_api_path(&path_join(&[BP_SYSTEM, "routes", "duplicate"])))
            .await
    }

    async fn read_raw(&self, uri: &str) -> Result<Vec<u8>> {
        let resp = self.api.get(Request::new(uri.to_string())).await?;

        let body = resp.bytes().await.map_err(|source| Error::Http {
            context: format!("reading response body from {uri}"),
            source,
        })?;

        Ok(body.to_vec())
    }

    /// Returns a single scheduled task.
    pub async fn get_system_task(&self, id: i64) -> Result<SystemTask> {
        self.api
            .get_into(Request::new(path_join(&[BP_SYSTEM, "task", &str_val(id)])))
            .await
    }
}
