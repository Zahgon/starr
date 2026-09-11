use super::Radarr;
use crate::error::Result;
use crate::helpers::{force_save, str_val};
use crate::interface::ApiClientExt;
use crate::is_default;
use crate::req::{Request, path_join};
use crate::shared::{FieldInput, FieldOutput, Protocol};
use serde::{Deserialize, Serialize};

const BP_DOWNLOAD_CLIENT: &str = "v3/downloadClient";

/// DownloadClientInput is the input for a new or updated download client.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DownloadClientInput {
    /// Whether the client is enabled.
    #[serde(default)]
    pub enable: bool,
    /// Whether completed downloads are removed from the client.
    #[serde(default)]
    pub remove_completed_downloads: bool,
    /// Whether failed downloads are removed from the client.
    #[serde(default)]
    pub remove_failed_downloads: bool,
    /// Client priority.
    #[serde(default)]
    pub priority: i32,
    /// Client ID. Leave zero when creating.
    #[serde(default, skip_serializing_if = "is_default")]
    pub id: i64,
    /// Configuration contract name.
    #[serde(default)]
    pub config_contract: String,
    /// Implementation type of the client.
    #[serde(default)]
    pub implementation: String,
    /// Client name.
    #[serde(default)]
    pub name: String,
    /// Protocol the client handles.
    #[serde(default)]
    pub protocol: Protocol,
    /// Tags applied to this client.
    #[serde(default)]
    pub tags: Vec<i32>,
    /// Implementation-specific settings.
    #[serde(default)]
    pub fields: Vec<FieldInput>,
}

/// DownloadClientOutput is the output from the download client methods.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DownloadClientOutput {
    /// Whether the client is enabled.
    #[serde(default)]
    pub enable: bool,
    /// Whether completed downloads are removed from the client.
    #[serde(default)]
    pub remove_completed_downloads: bool,
    /// Whether failed downloads are removed from the client.
    #[serde(default)]
    pub remove_failed_downloads: bool,
    /// Client priority.
    #[serde(default)]
    pub priority: i32,
    /// Client ID.
    #[serde(default, skip_serializing_if = "is_default")]
    pub id: i64,
    /// Configuration contract name.
    #[serde(default)]
    pub config_contract: String,
    /// Implementation type of the client.
    #[serde(default)]
    pub implementation: String,
    /// Display name of the implementation.
    #[serde(default)]
    pub implementation_name: String,
    /// Link to more information about the client.
    #[serde(default)]
    pub info_link: String,
    /// Client name.
    #[serde(default)]
    pub name: String,
    /// Protocol the client handles.
    #[serde(default)]
    pub protocol: Protocol,
    /// Tags applied to this client.
    #[serde(default)]
    pub tags: Vec<i32>,
    /// Implementation-specific settings.
    #[serde(default)]
    pub fields: Vec<FieldOutput>,
}

impl Radarr {
    /// Returns all configured download clients.
    pub async fn get_download_clients(&self) -> Result<Vec<DownloadClientOutput>> {
        self.api.get_into(Request::new(BP_DOWNLOAD_CLIENT)).await
    }

    /// Returns a single download client.
    pub async fn get_download_client(
        &self,
        downloadclient_id: i64,
    ) -> Result<DownloadClientOutput> {
        self.api
            .get_into(Request::new(path_join(&[
                BP_DOWNLOAD_CLIENT,
                &str_val(downloadclient_id),
            ])))
            .await
    }

    /// Creates a download client without testing it.
    pub async fn add_download_client(
        &self,
        client: &DownloadClientInput,
    ) -> Result<DownloadClientOutput> {
        let mut client = client.clone();
        client.id = 0;

        let req = Request::new(BP_DOWNLOAD_CLIENT)
            .with_json(&client)?
            .with_query(force_save(true));
        self.api.post_into(req).await
    }

    /// Tests a download client.
    pub async fn test_download_client(&self, client: &DownloadClientInput) -> Result<()> {
        let req = Request::new(path_join(&[BP_DOWNLOAD_CLIENT, "test"])).with_json(client)?;
        self.api.post_any(req).await
    }

    /// Updates the download client.
    pub async fn update_download_client(
        &self,
        client: &DownloadClientInput,
        force: bool,
    ) -> Result<DownloadClientOutput> {
        let req = Request::new(path_join(&[BP_DOWNLOAD_CLIENT, &str_val(client.id)]))
            .with_json(client)?
            .with_query(force_save(force));
        self.api.put_into(req).await
    }

    /// Removes a single download client.
    pub async fn delete_download_client(&self, downloadclient_id: i64) -> Result<()> {
        self.api
            .delete_any(Request::new(path_join(&[
                BP_DOWNLOAD_CLIENT,
                &str_val(downloadclient_id),
            ])))
            .await
    }
}
