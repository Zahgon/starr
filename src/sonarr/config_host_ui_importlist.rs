use super::Sonarr;
use crate::error::Result;
use crate::helpers::str_val;
use crate::interface::ApiClientExt;
use crate::is_default;
use crate::req::{Request, path_join};
use serde::{Deserialize, Serialize};

const BP_CONFIG_HOST: &str = "v3/config/host";
const BP_CONFIG_UI: &str = "v3/config/ui";
const BP_CONFIG_IMPORT_LIST: &str = "v3/config/importlist";

/// HostConfig is the `/api/v3/config/host` resource.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HostConfig {
    /// Config ID.
    #[serde(default, skip_serializing_if = "is_default")]
    pub id: i32,
    /// Address the server binds to.
    #[serde(default, skip_serializing_if = "is_default")]
    pub bind_address: String,
    /// Port the server listens on.
    #[serde(default)]
    pub port: i32,
    /// Port the server listens on for TLS.
    #[serde(default, rename = "sslPort")]
    pub ssl_port: i32,
    /// Whether TLS is enabled.
    #[serde(default, rename = "enableSsl")]
    pub enable_ssl: bool,
    /// Whether a browser opens on start.
    #[serde(default)]
    pub launch_browser: bool,
    /// Authentication method in use.
    #[serde(default, skip_serializing_if = "is_default")]
    pub authentication_method: String,
    /// When authentication is required.
    #[serde(default, skip_serializing_if = "is_default")]
    pub authentication_required: String,
    /// Whether analytics are enabled.
    #[serde(default)]
    pub analytics_enabled: bool,
    /// Login user name.
    #[serde(default, skip_serializing_if = "is_default")]
    pub username: String,
    /// Login password.
    #[serde(default, skip_serializing_if = "is_default")]
    pub password: String,
    /// Login password confirmation.
    #[serde(default, skip_serializing_if = "is_default")]
    pub password_confirmation: String,
    /// Level written to the log.
    #[serde(default, skip_serializing_if = "is_default")]
    pub log_level: String,
    /// Maximum log size in megabytes.
    #[serde(default)]
    pub log_size_limit: i32,
    /// Level written to the console.
    #[serde(default, skip_serializing_if = "is_default")]
    pub console_log_level: String,
    /// Branch the app updates from.
    #[serde(default, skip_serializing_if = "is_default")]
    pub branch: String,
    /// API key for this instance.
    #[serde(default, skip_serializing_if = "is_default", rename = "apiKey")]
    pub api_key: String,
    /// Path to the TLS certificate.
    #[serde(default, skip_serializing_if = "is_default", rename = "sslCertPath")]
    pub ssl_cert_path: String,
    /// Password for the TLS certificate.
    #[serde(
        default,
        skip_serializing_if = "is_default",
        rename = "sslCertPassword"
    )]
    pub ssl_cert_password: String,
    /// URL base the app is served under.
    #[serde(default, skip_serializing_if = "is_default", rename = "urlBase")]
    pub url_base: String,
    /// Name given to this instance.
    #[serde(default, skip_serializing_if = "is_default")]
    pub instance_name: String,
    /// External URL of this instance.
    #[serde(
        default,
        skip_serializing_if = "is_default",
        rename = "applicationUrl"
    )]
    pub application_url: String,
    /// Whether updates install automatically.
    #[serde(default)]
    pub update_automatically: bool,
    /// How the app updates itself.
    #[serde(default, skip_serializing_if = "is_default")]
    pub update_mechanism: String,
    /// Script that performs the update.
    #[serde(default, skip_serializing_if = "is_default")]
    pub update_script_path: String,
    /// Whether a proxy is used.
    #[serde(default)]
    pub proxy_enabled: bool,
    /// Type of the proxy.
    #[serde(default, skip_serializing_if = "is_default")]
    pub proxy_type: String,
    /// Host name of the proxy.
    #[serde(default, skip_serializing_if = "is_default")]
    pub proxy_hostname: String,
    /// Port of the proxy.
    #[serde(default)]
    pub proxy_port: i32,
    /// User name for the proxy.
    #[serde(default, skip_serializing_if = "is_default")]
    pub proxy_username: String,
    /// Password for the proxy.
    #[serde(default, skip_serializing_if = "is_default")]
    pub proxy_password: String,
    /// Hosts that bypass the proxy.
    #[serde(default, skip_serializing_if = "is_default")]
    pub proxy_bypass_filter: String,
    /// Whether local addresses bypass the proxy.
    #[serde(default)]
    pub proxy_bypass_local_addresses: bool,
    /// How certificates are validated.
    #[serde(default, skip_serializing_if = "is_default")]
    pub certificate_validation: String,
    /// Folder backups are written to.
    #[serde(default, skip_serializing_if = "is_default")]
    pub backup_folder: String,
    /// Days between backups.
    #[serde(default)]
    pub backup_interval: i32,
    /// Number of backups kept.
    #[serde(default)]
    pub backup_retention: i32,
    /// Whether CGNAT addresses are trusted.
    #[serde(default, rename = "trustCgnatIpAddresses")]
    pub trust_cgnat_ip_addresses: bool,
}

/// UIConfig is the `/api/v3/config/ui` resource.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UIConfig {
    /// Config ID.
    #[serde(default, skip_serializing_if = "is_default")]
    pub id: i32,
    /// First day of the week in the calendar.
    #[serde(default)]
    pub first_day_of_week: i32,
    /// Header format of the calendar week column.
    #[serde(default, skip_serializing_if = "is_default")]
    pub calendar_week_column_header: String,
    /// Format used for short dates.
    #[serde(default, skip_serializing_if = "is_default")]
    pub short_date_format: String,
    /// Format used for long dates.
    #[serde(default, skip_serializing_if = "is_default")]
    pub long_date_format: String,
    /// Format used for times.
    #[serde(default, skip_serializing_if = "is_default")]
    pub time_format: String,
    /// Whether relative dates are shown.
    #[serde(default)]
    pub show_relative_dates: bool,
    /// Whether color impaired mode is on.
    #[serde(default)]
    pub enable_color_impaired_mode: bool,
    /// UI theme.
    #[serde(default, skip_serializing_if = "is_default")]
    pub theme: String,
    /// Language of the UI.
    #[serde(default, rename = "uiLanguage")]
    pub ui_language: i32,
}

/// ImportListConfig is the `/api/v3/config/importlist` resource.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ImportListConfig {
    /// Config ID.
    #[serde(default, skip_serializing_if = "is_default")]
    pub id: i32,
    /// What list sync does to items no longer on a list.
    #[serde(default, skip_serializing_if = "is_default")]
    pub list_sync_level: String,
    /// Tag applied by list sync.
    #[serde(default)]
    pub list_sync_tag: i32,
}

impl Sonarr {
    /// Returns the host config.
    pub async fn get_host_config(&self) -> Result<HostConfig> {
        self.api.get_into(Request::new(BP_CONFIG_HOST)).await
    }

    /// Returns the host config by id.
    pub async fn get_host_config_by_id(&self, id: i32) -> Result<HostConfig> {
        self.api
            .get_into(Request::new(path_join(&[BP_CONFIG_HOST, &str_val(id)])))
            .await
    }

    /// Updates the host config.
    pub async fn update_host_config(&self, input: &HostConfig) -> Result<HostConfig> {
        let req =
            Request::new(path_join(&[BP_CONFIG_HOST, &str_val(input.id)])).with_json(input)?;
        self.api.put_into(req).await
    }

    /// Returns the UI config.
    pub async fn get_ui_config(&self) -> Result<UIConfig> {
        self.api.get_into(Request::new(BP_CONFIG_UI)).await
    }

    /// Returns the UI config by id.
    pub async fn get_ui_config_by_id(&self, id: i32) -> Result<UIConfig> {
        self.api
            .get_into(Request::new(path_join(&[BP_CONFIG_UI, &str_val(id)])))
            .await
    }

    /// Updates the UI config.
    pub async fn update_ui_config(&self, input: &UIConfig) -> Result<UIConfig> {
        let req = Request::new(path_join(&[BP_CONFIG_UI, &str_val(input.id)])).with_json(input)?;
        self.api.put_into(req).await
    }

    /// Returns the import list config.
    pub async fn get_import_list_config(&self) -> Result<ImportListConfig> {
        self.api.get_into(Request::new(BP_CONFIG_IMPORT_LIST)).await
    }

    /// Returns the import list config by id.
    pub async fn get_import_list_config_by_id(&self, id: i32) -> Result<ImportListConfig> {
        self.api
            .get_into(Request::new(path_join(&[
                BP_CONFIG_IMPORT_LIST,
                &str_val(id),
            ])))
            .await
    }

    /// Updates the import list config.
    pub async fn update_import_list_config(
        &self,
        input: &ImportListConfig,
    ) -> Result<ImportListConfig> {
        let req = Request::new(path_join(&[BP_CONFIG_IMPORT_LIST, &str_val(input.id)]))
            .with_json(input)?;
        self.api.put_into(req).await
    }
}
