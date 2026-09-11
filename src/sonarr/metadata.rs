use super::Sonarr;
use crate::error::Result;
use crate::helpers::{force_save, str_val};
use crate::interface::ApiClientExt;
use crate::is_default;
use crate::req::{Request, path_clean, path_join};
use crate::shared::{FieldInput, FieldOutput};
use crate::values::Values;
use serde::{Deserialize, Serialize};

const BP_METADATA: &str = "v3/metadata";

/// MetadataProviderMessage is the provider message object on metadata consumers.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MetadataProviderMessage {
    /// The message text.
    #[serde(default, skip_serializing_if = "is_default")]
    pub message: String,
    /// The message type.
    #[serde(default, skip_serializing_if = "is_default", rename = "type")]
    pub message_type: String,
}

/// MetadataOutput is the output from `/api/v3/metadata` (MetadataResource).
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MetadataOutput {
    /// Consumer ID.
    #[serde(default, skip_serializing_if = "is_default")]
    pub id: i64,
    /// Consumer name.
    #[serde(default, skip_serializing_if = "is_default")]
    pub name: String,
    /// Implementation-specific settings.
    #[serde(default, skip_serializing_if = "is_default")]
    pub fields: Vec<FieldOutput>,
    /// Display name of the implementation.
    #[serde(default, skip_serializing_if = "is_default")]
    pub implementation_name: String,
    /// Implementation type of the consumer.
    #[serde(default, skip_serializing_if = "is_default")]
    pub implementation: String,
    /// Configuration contract name.
    #[serde(default, skip_serializing_if = "is_default")]
    pub config_contract: String,
    /// Link to more information about the consumer.
    #[serde(default, skip_serializing_if = "is_default")]
    pub info_link: String,
    /// Status message attached to the consumer.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<MetadataProviderMessage>,
    /// Tags applied to this consumer.
    #[serde(default, skip_serializing_if = "is_default")]
    pub tags: Vec<i32>,
    /// Templates for this consumer.
    #[serde(default, skip_serializing_if = "is_default")]
    pub presets: Vec<MetadataOutput>,
    /// Whether the consumer is enabled.
    #[serde(default)]
    pub enable: bool,
}

/// MetadataInput is the input for creating or updating metadata consumers.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MetadataInput {
    /// Consumer ID. Leave zero when creating.
    #[serde(default, skip_serializing_if = "is_default")]
    pub id: i64,
    /// Consumer name.
    #[serde(default, skip_serializing_if = "is_default")]
    pub name: String,
    /// Implementation-specific settings.
    #[serde(default, skip_serializing_if = "is_default")]
    pub fields: Vec<FieldInput>,
    /// Implementation type of the consumer.
    #[serde(default, skip_serializing_if = "is_default")]
    pub implementation: String,
    /// Configuration contract name.
    #[serde(default, skip_serializing_if = "is_default")]
    pub config_contract: String,
    /// Tags applied to this consumer.
    #[serde(default, skip_serializing_if = "is_default")]
    pub tags: Vec<i32>,
    /// Whether the consumer is enabled.
    #[serde(default)]
    pub enable: bool,
}

impl Sonarr {
    /// Returns all configured metadata consumers.
    pub async fn get_metadata(&self) -> Result<Vec<MetadataOutput>> {
        self.api.get_into(Request::new(BP_METADATA)).await
    }

    /// Returns a single metadata consumer.
    pub async fn get_metadata_by_id(&self, id: i64) -> Result<MetadataOutput> {
        self.api
            .get_into(Request::new(path_join(&[BP_METADATA, &str_val(id)])))
            .await
    }

    /// Returns metadata consumer templates.
    pub async fn get_metadata_schema(&self) -> Result<Vec<MetadataOutput>> {
        self.api
            .get_into(Request::new(path_join(&[BP_METADATA, "schema"])))
            .await
    }

    /// Creates a metadata consumer.
    pub async fn add_metadata(
        &self,
        input: &MetadataInput,
        force_save_input: bool,
    ) -> Result<MetadataOutput> {
        let req = Request::new(BP_METADATA)
            .with_json(input)?
            .with_query(force_save(force_save_input));
        self.api.post_into(req).await
    }

    /// Updates a metadata consumer.
    pub async fn update_metadata(
        &self,
        input: &MetadataInput,
        force_save_input: bool,
    ) -> Result<MetadataOutput> {
        let req = Request::new(path_join(&[BP_METADATA, &str_val(input.id)]))
            .with_json(input)?
            .with_query(force_save(force_save_input));
        self.api.put_into(req).await
    }

    /// Deletes a metadata consumer.
    pub async fn delete_metadata(&self, id: i64) -> Result<()> {
        self.api
            .delete_any(Request::new(path_join(&[BP_METADATA, &str_val(id)])))
            .await
    }

    /// Runs a named action on a metadata consumer.
    pub async fn metadata_action(&self, name: &str, input: &MetadataInput) -> Result<()> {
        let action = path_base(name);
        let req = Request::new(path_join(&[BP_METADATA, "action", &action])).with_json(input)?;
        self.api.post_any(req).await
    }

    /// Tests a metadata consumer configuration.
    pub async fn test_metadata(&self, input: &MetadataInput, force_test: bool) -> Result<()> {
        let mut query = Values::new();
        if force_test {
            query.set("forceTest", "true");
        }

        let req = Request::new(path_join(&[BP_METADATA, "test"]))
            .with_json(input)?
            .with_query(query);
        self.api.post_any(req).await
    }

    /// Tests all metadata consumers.
    pub async fn test_all_metadata(&self) -> Result<()> {
        self.api
            .post_any(Request::new(path_join(&[BP_METADATA, "testall"])))
            .await
    }
}

/// Returns the last element of a path, like Go's `path.Base`.
fn path_base(name: &str) -> String {
    let cleaned = path_clean(name);

    match cleaned.rsplit('/').next() {
        Some("") => "/".to_string(),
        Some(base) => base.to_string(),
        None => cleaned,
    }
}
