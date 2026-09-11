use super::Lidarr;
use crate::error::Result;
use crate::helpers::{force_save, str_val};
use crate::interface::ApiClientExt;
use crate::is_default;
use crate::req::{Request, path_join};
use crate::shared::{FieldInput, FieldOutput};
use serde::{Deserialize, Serialize};

const BP_IMPORT_LIST: &str = "v1/importlist";

/// ImportListInput is the input for a new or updated import list.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ImportListInput {
    /// Whether items are added automatically.
    #[serde(default)]
    pub enable_automatic_add: bool,
    /// Whether existing items are monitored.
    #[serde(default)]
    pub should_monitor_existing: bool,
    /// Whether added items are searched for.
    #[serde(default)]
    pub should_search: bool,
    /// Order this list is applied in.
    #[serde(default)]
    pub list_order: i32,
    /// List ID. For update, not add.
    #[serde(default, skip_serializing_if = "is_default")]
    pub id: i64,
    /// Quality profile applied to added items.
    #[serde(default, skip_serializing_if = "is_default")]
    pub quality_profile_id: i64,
    /// Metadata profile applied to added items.
    #[serde(default, skip_serializing_if = "is_default")]
    pub metadata_profile_id: i64,
    /// Configuration contract name.
    #[serde(default, skip_serializing_if = "is_default")]
    pub config_contract: String,
    /// Implementation type of the list.
    #[serde(default, skip_serializing_if = "is_default")]
    pub implementation: String,
    /// Type of the list.
    #[serde(default, skip_serializing_if = "is_default")]
    pub list_type: String,
    /// How new items are monitored.
    #[serde(default, skip_serializing_if = "is_default")]
    pub monitor_new_items: String,
    /// List name.
    #[serde(default, skip_serializing_if = "is_default")]
    pub name: String,
    /// Root folder added items are placed in.
    #[serde(default, skip_serializing_if = "is_default")]
    pub root_folder_path: String,
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

/// ImportListOutput is the output from the import list methods.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ImportListOutput {
    /// Whether items are added automatically.
    #[serde(default)]
    pub enable_automatic_add: bool,
    /// Whether existing items are monitored.
    #[serde(default)]
    pub should_monitor_existing: bool,
    /// Whether added items are searched for.
    #[serde(default)]
    pub should_search: bool,
    /// Order this list is applied in.
    #[serde(default)]
    pub list_order: i32,
    /// List ID.
    #[serde(default)]
    pub id: i64,
    /// Quality profile applied to added items.
    #[serde(default)]
    pub quality_profile_id: i64,
    /// Metadata profile applied to added items.
    #[serde(default)]
    pub metadata_profile_id: i64,
    /// What gets monitored on added items.
    #[serde(default)]
    pub should_monitor: String,
    /// Root folder added items are placed in.
    #[serde(default)]
    pub root_folder_path: String,
    /// How new items are monitored.
    #[serde(default)]
    pub monitor_new_items: String,
    /// Type of the list.
    #[serde(default)]
    pub list_type: String,
    /// List name.
    #[serde(default)]
    pub name: String,
    /// Display name of the implementation.
    #[serde(default)]
    pub implementation_name: String,
    /// Implementation type of the list.
    #[serde(default)]
    pub implementation: String,
    /// Configuration contract name.
    #[serde(default)]
    pub config_contract: String,
    /// Link to more information about the list.
    #[serde(default)]
    pub info_link: String,
    /// Tags applied to this list.
    #[serde(default)]
    pub tags: Vec<i32>,
    /// Implementation-specific settings.
    #[serde(default)]
    pub fields: Vec<FieldOutput>,
    /// Status message attached to the list.
    #[serde(default)]
    pub message: ImportListMessage,
}

/// ImportListMessage is the status message embedded in an [`ImportListOutput`].
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ImportListMessage {
    /// The message text.
    #[serde(default)]
    pub message: String,
    /// The message type.
    #[serde(default, rename = "type")]
    pub message_type: String,
}

impl Lidarr {
    /// Returns all configured import lists.
    pub async fn get_import_lists(&self) -> Result<Vec<ImportListOutput>> {
        self.api.get_into(Request::new(BP_IMPORT_LIST)).await
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

    /// Creates an import list without testing it.
    pub async fn add_import_list(
        &self,
        import_list: &ImportListInput,
    ) -> Result<ImportListOutput> {
        let mut import_list = import_list.clone();
        import_list.id = 0;

        let req = Request::new(BP_IMPORT_LIST)
            .with_json(&import_list)?
            .with_query(force_save(true));
        self.api.post_into(req).await
    }

    /// Tests an import list.
    pub async fn test_import_list(&self, list: &ImportListInput) -> Result<()> {
        let req = Request::new(path_join(&[BP_IMPORT_LIST, "test"])).with_json(list)?;
        self.api.post_any(req).await
    }

    /// Updates the import list.
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

    /// Removes a single import list.
    pub async fn delete_import_list(&self, import_list_id: i64) -> Result<()> {
        self.api
            .delete_any(Request::new(path_join(&[
                BP_IMPORT_LIST,
                &str_val(import_list_id),
            ])))
            .await
    }
}
