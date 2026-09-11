use super::Radarr;
use crate::error::Result;
use crate::helpers::str_val;
use crate::interface::ApiClientExt;
use crate::is_default;
use crate::req::{Request, path_join};
use crate::shared::{FieldInput, FieldOutput};
use serde::{Deserialize, Serialize};

const BP_CUSTOM_FORMAT: &str = "v3/customFormat";

/// CustomFormatInput is the input for a new or updated [`CustomFormatOutput`].
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CustomFormatInput {
    /// Custom format ID. Leave zero when creating.
    #[serde(default, skip_serializing_if = "is_default")]
    pub id: i64,
    /// Custom format name.
    #[serde(default)]
    pub name: String,
    /// Whether the format name is included when renaming files.
    #[serde(default, rename = "includeCustomFormatWhenRenaming")]
    pub include_cf_when_renaming: bool,
    /// Rules that make up this custom format.
    #[serde(default)]
    pub specifications: Vec<CustomFormatInputSpec>,
}

/// CustomFormatInputSpec is part of a [`CustomFormatInput`].
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CustomFormatInputSpec {
    /// Specification name.
    #[serde(default)]
    pub name: String,
    /// Implementation type of the specification.
    #[serde(default)]
    pub implementation: String,
    /// Whether the match is inverted.
    #[serde(default)]
    pub negate: bool,
    /// Whether this specification must match.
    #[serde(default)]
    pub required: bool,
    /// Implementation-specific settings.
    #[serde(default)]
    pub fields: Vec<FieldInput>,
}

/// CustomFormatOutput is the output from the custom format methods.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CustomFormatOutput {
    /// Custom format ID.
    #[serde(default)]
    pub id: i64,
    /// Custom format name.
    #[serde(default)]
    pub name: String,
    /// Whether the format name is included when renaming files.
    #[serde(default, rename = "includeCustomFormatWhenRenaming")]
    pub include_cf_when_renaming: bool,
    /// Rules that make up this custom format.
    #[serde(default)]
    pub specifications: Vec<CustomFormatOutputSpec>,
}

/// CustomFormatOutputSpec is part of a [`CustomFormatOutput`].
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CustomFormatOutputSpec {
    /// Specification name.
    #[serde(default)]
    pub name: String,
    /// Implementation type of the specification.
    #[serde(default)]
    pub implementation: String,
    /// Display name of the implementation.
    #[serde(default)]
    pub implementation_name: String,
    /// Link to documentation for this specification.
    #[serde(default)]
    pub info_link: String,
    /// Whether the match is inverted.
    #[serde(default)]
    pub negate: bool,
    /// Whether this specification must match.
    #[serde(default)]
    pub required: bool,
    /// Implementation-specific settings.
    #[serde(default)]
    pub fields: Vec<FieldOutput>,
}

impl Radarr {
    /// Returns all configured custom formats.
    pub async fn get_custom_formats(&self) -> Result<Vec<CustomFormatOutput>> {
        self.api.get_into(Request::new(BP_CUSTOM_FORMAT)).await
    }

    /// Returns a single custom format.
    pub async fn get_custom_format(&self, customformat_id: i64) -> Result<CustomFormatOutput> {
        self.api
            .get_into(Request::new(path_join(&[
                BP_CUSTOM_FORMAT,
                &str_val(customformat_id),
            ])))
            .await
    }

    /// Creates a new custom format and returns the response (with ID).
    pub async fn add_custom_format(
        &self,
        format: &CustomFormatInput,
    ) -> Result<CustomFormatOutput> {
        self.api
            .post_into(Request::new(BP_CUSTOM_FORMAT).with_json(format)?)
            .await
    }

    /// Updates an existing custom format and returns the response.
    pub async fn update_custom_format(
        &self,
        format: &CustomFormatInput,
    ) -> Result<CustomFormatOutput> {
        let req = Request::new(path_join(&[BP_CUSTOM_FORMAT, &str_val(format.id)]))
            .with_json(format)?;
        self.api.put_into(req).await
    }

    /// Deletes a custom format.
    pub async fn delete_custom_format(&self, cf_id: i64) -> Result<()> {
        self.api
            .delete_any(Request::new(path_join(&[
                BP_CUSTOM_FORMAT,
                &str_val(cf_id),
            ])))
            .await
    }
}
