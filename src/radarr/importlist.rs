use super::{Availability, Radarr};
use crate::error::{Error, Result};
use crate::helpers::{force_save, str_val};
use crate::interface::ApiClientExt;
use crate::is_default;
use crate::req::{Request, path_join};
use crate::shared::{FieldInput, FieldOutput};
use serde::{Deserialize, Serialize};
use std::fmt::Write;

const BP_IMPORT_LIST: &str = "v3/importlist";

/// ImportListInput represents the `api/v3/importlist` endpoint.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ImportListInput {
    /// Whether items are added automatically.
    #[serde(default)]
    pub enable_auto: bool,
    /// Whether the list is enabled.
    #[serde(default)]
    pub enabled: bool,
    /// Whether added items are searched for.
    #[serde(default)]
    pub search_on_add: bool,
    /// Order this list is applied in.
    #[serde(default)]
    pub list_order: i32,
    /// List ID. Leave zero when creating.
    #[serde(default, skip_serializing_if = "is_default")]
    pub id: i64,
    /// Quality profile applied to added items.
    #[serde(default, skip_serializing_if = "is_default")]
    pub quality_profile_id: i64,
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
    /// What gets monitored on added items.
    #[serde(default, skip_serializing_if = "is_default")]
    pub monitor: String,
    /// List name.
    #[serde(default, skip_serializing_if = "is_default")]
    pub name: String,
    /// Root folder added items are placed in.
    #[serde(default, skip_serializing_if = "is_default")]
    pub root_folder_path: String,
    /// Minimum availability applied to added items.
    #[serde(default, skip_serializing_if = "is_default")]
    pub minimum_availability: Availability,
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
    pub enable_auto: bool,
    /// Whether the list is enabled.
    #[serde(default)]
    pub enabled: bool,
    /// Whether added items are searched for.
    #[serde(default)]
    pub search_on_add: bool,
    /// List ID.
    #[serde(default)]
    pub id: i64,
    /// Order this list is applied in.
    #[serde(default)]
    pub list_order: i64,
    /// Quality profile applied to added items.
    #[serde(default)]
    pub quality_profile_id: i64,
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
    /// What gets monitored on added items.
    #[serde(default)]
    pub monitor: String,
    /// Type of the list.
    #[serde(default)]
    pub list_type: String,
    /// List name.
    #[serde(default)]
    pub name: String,
    /// Root folder added items are placed in.
    #[serde(default)]
    pub root_folder_path: String,
    /// Minimum availability applied to added items.
    #[serde(default)]
    pub minimum_availability: Availability,
    /// Tags applied to this list.
    #[serde(default)]
    pub tags: Vec<i32>,
    /// Implementation-specific settings.
    #[serde(default)]
    pub fields: Vec<FieldOutput>,
}

impl Radarr {
    /// Returns all import lists.
    pub async fn get_import_lists(&self) -> Result<Vec<ImportListOutput>> {
        self.api.get_into(Request::new(BP_IMPORT_LIST)).await
    }

    /// Creates an import list in Radarr without testing it.
    pub async fn add_import_list(&self, list: &ImportListInput) -> Result<ImportListOutput> {
        let mut list = list.clone();
        list.id = 0;

        let req = Request::new(BP_IMPORT_LIST)
            .with_json(&list)?
            .with_query(force_save(true));
        self.api.post_into(req).await
    }

    /// Removes import lists from Radarr.
    pub async fn delete_import_list(&self, ids: &[i64]) -> Result<()> {
        let mut errs = String::new();

        for id in ids {
            let uri = path_join(&[BP_IMPORT_LIST, &str_val(*id)]);
            if let Err(err) = self.api.delete_any(Request::new(uri.clone())).await {
                let _ = write!(errs, "api.Delete({uri}): {err} ");
            }
        }

        if errs.is_empty() {
            return Ok(());
        }

        Err(Error::Request(errs))
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
