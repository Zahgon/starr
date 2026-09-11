use super::Radarr;
use crate::error::Result;
use crate::helpers::str_val;
use crate::interface::ApiClientExt;
use crate::req::{Request, path_join};
use serde::{Deserialize, Serialize};

const BP_DOWNLOAD_CLIENT_CONFIG: &str = "v3/config/downloadClient";

/// DownloadClientConfig is the `/api/v3/config/downloadClient` endpoint.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DownloadClientConfig {
    /// Whether completed downloads are handled.
    #[serde(default)]
    pub enable_completed_download_handling: bool,
    /// Whether failed downloads are grabbed again automatically.
    #[serde(default)]
    pub auto_redownload_failed: bool,
    /// Minutes between checks for finished downloads.
    #[serde(default)]
    pub check_for_finished_download_interval: i64,
    /// Config ID.
    #[serde(default)]
    pub id: i64,
    /// Folders the download client works in.
    #[serde(default)]
    pub download_client_working_folders: String,
}

impl Radarr {
    /// Returns the download client config.
    pub async fn get_download_client_config(&self) -> Result<DownloadClientConfig> {
        self.api
            .get_into(Request::new(BP_DOWNLOAD_CLIENT_CONFIG))
            .await
    }

    /// Updates the single download client config.
    pub async fn update_download_client_config(
        &self,
        config: &DownloadClientConfig,
    ) -> Result<DownloadClientConfig> {
        let req = Request::new(path_join(&[
            BP_DOWNLOAD_CLIENT_CONFIG,
            &str_val(config.id),
        ]))
        .with_json(config)?;
        self.api.put_into(req).await
    }
}
