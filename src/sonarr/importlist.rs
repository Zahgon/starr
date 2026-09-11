use super::Sonarr;
use crate::error::Result;
use crate::helpers::{force_save, str_val};
use crate::interface::ApiClientExt;
use crate::is_default;
use crate::req::{Request, path_join};
use crate::shared::{FieldInput, FieldOutput};
use serde::{Deserialize, Serialize};

const BP_IMPORT_LIST: &str = "v3/importList";

/// ImportListInput represents the `api/v3/importlist` endpoint.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ImportListInput {
    /// Whether items are added automatically.
    #[serde(default)]
    pub enable_automatic_add: bool,
    /// Whether episodes go into season folders.
    #[serde(default)]
    pub season_folder: bool,
    /// Order this list is applied in.
    #[serde(default)]
    pub list_order: i32,
    /// Quality profile applied to added items.
    #[serde(default, skip_serializing_if = "is_default")]
    pub quality_profile_id: i64,
    /// List ID. Update only.
    #[serde(default, skip_serializing_if = "is_default")]
    pub id: i64,
    /// Configuration contract name.
    #[serde(default, skip_serializing_if = "is_default")]
    pub config_contract: String,
    /// Implementation type of the list.
    #[serde(default, skip_serializing_if = "is_default")]
    pub implementation: String,
    /// Display name of the implementation.
    #[serde(default, skip_serializing_if = "is_default")]
    pub implementation_name: String,
    /// Link to more information about the list.
    #[serde(default, skip_serializing_if = "is_default")]
    pub info_link: String,
    /// Type of the list.
    #[serde(default, skip_serializing_if = "is_default")]
    pub list_type: String,
    /// Shortest interval between refreshes.
    #[serde(default, skip_serializing_if = "is_default")]
    pub min_refresh_interval: String,
    /// List name.
    #[serde(default, skip_serializing_if = "is_default")]
    pub name: String,
    /// Root folder added items are placed in.
    #[serde(default, skip_serializing_if = "is_default")]
    pub root_folder_path: String,
    /// Type applied to added series.
    #[serde(default, skip_serializing_if = "is_default")]
    pub series_type: String,
    /// What gets monitored on added items.
    #[serde(default, skip_serializing_if = "is_default")]
    pub should_monitor: String,
    /// Tags applied to this list.
    #[serde(default, skip_serializing_if = "is_default")]
    pub tags: Vec<i32>,
    /// Implementation-specific settings.
    #[serde(default, skip_serializing_if = "is_default")]
    pub fields: Vec<FieldInput>,
}

/// ImportListOutput represents the `api/v3/importlist` endpoint.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ImportListOutput {
    /// Whether items are added automatically.
    #[serde(default)]
    pub enable_automatic_add: bool,
    /// Whether episodes go into season folders.
    #[serde(default)]
    pub season_folder: bool,
    /// Quality profile applied to added items.
    #[serde(default)]
    pub quality_profile_id: i64,
    /// Order this list is applied in.
    #[serde(default)]
    pub list_order: i64,
    /// List ID.
    #[serde(default)]
    pub id: i64,
    /// Configuration contract name.
    #[serde(default)]
    pub config_contract: String,
    /// Implementation type of the list.
    #[serde(default)]
    pub implementation: String,
    /// Display name of the implementation.
    #[serde(default)]
    pub implementation_name: String,
    /// Link to more information about the list.
    #[serde(default)]
    pub info_link: String,
    /// Type of the list.
    #[serde(default)]
    pub list_type: String,
    /// Shortest interval between refreshes.
    #[serde(default)]
    pub min_refresh_interval: String,
    /// List name.
    #[serde(default)]
    pub name: String,
    /// Root folder added items are placed in.
    #[serde(default)]
    pub root_folder_path: String,
    /// Type applied to added series.
    #[serde(default)]
    pub series_type: String,
    /// What gets monitored on added items.
    #[serde(default)]
    pub should_monitor: String,
    /// Tags applied to this list.
    #[serde(default)]
    pub tags: Vec<i32>,
    /// Implementation-specific settings.
    #[serde(default)]
    pub fields: Vec<FieldOutput>,
}

impl Sonarr {
    /// Returns all import lists.
    pub async fn get_import_lists(&self) -> Result<Vec<ImportListOutput>> {
        self.api.get_into(Request::new(BP_IMPORT_LIST)).await
    }

    /// Creates an import list in Sonarr without testing it.
    pub async fn add_import_list(&self, list: &ImportListInput) -> Result<ImportListOutput> {
        let mut list = list.clone();
        list.id = 0;

        let req = Request::new(BP_IMPORT_LIST)
            .with_json(&list)?
            .with_query(force_save(true));
        self.api.post_into(req).await
    }

    /// Returns a single import list.
    pub async fn get_import_list(&self, import_list_id: i64) -> Result<ImportListOutput> {
        self.api
            .get_into(Request::new(path_join(&[
                BP_IMPORT_LIST,
                &str_val(import_list_id),
            ])))
            .await
    }

    /// Removes a single import list.
    pub async fn delete_import_list(&self, import_list_id: i64) -> Result<()> {
        self.api
            .delete_any(Request::new(path_join(&[
                BP_IMPORT_LIST,
                &str_val(import_list_id),
            ])))
            .await
    }

    /// Tests an import list.
    pub async fn test_import_list(&self, list: &ImportListInput) -> Result<()> {
        let req = Request::new(path_join(&[BP_IMPORT_LIST, "test"])).with_json(list)?;
        self.api.post_any(req).await
    }

    /// Updates an existing import list and returns the response.
    pub async fn update_import_list(
        &self,
        import_list: &ImportListInput,
        force: bool,
    ) -> Result<ImportListOutput> {
        let req = Request::new(path_join(&[BP_IMPORT_LIST, &str_val(import_list.id)]))
            .with_json(import_list)?
            .with_query(force_save(force));
        self.api.put_into(req).await
    }
}
