use super::Readarr;
use crate::error::Result;
use crate::helpers::str_val;
use crate::interface::ApiClientExt;
use crate::is_default;
use crate::req::{Request, path_join};
use crate::shared::BaseQuality;
use serde::{Deserialize, Serialize};

const BP_QUALITY_DEFINITION: &str = "v1/qualityDefinition";

/// QualityDefinition is the `/api/v1/qualitydefinition` endpoint.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct QualityDefinition {
    /// Definition ID.
    #[serde(default, skip_serializing_if = "is_default")]
    pub id: i64,
    /// Weight of this definition. This should not be changed.
    #[serde(default)]
    pub weight: i64,
    /// Smallest accepted size, in megabytes per minute.
    #[serde(default)]
    pub min_size: f64,
    /// Largest accepted size, in megabytes per minute.
    #[serde(default)]
    pub max_size: f64,
    /// Preferred size, in megabytes per minute.
    #[serde(default, rename = "preferredSize")]
    pub pref_size: f64,
    /// Definition title.
    #[serde(default)]
    pub title: String,
    /// The quality this definition applies to.
    #[serde(default)]
    pub quality: Option<BaseQuality>,
}

impl Readarr {
    /// Returns all configured quality definitions.
    pub async fn get_quality_definitions(&self) -> Result<Vec<QualityDefinition>> {
        self.api
            .get_into(Request::new(BP_QUALITY_DEFINITION))
            .await
    }

    /// Returns a single quality definition.
    pub async fn get_quality_definition(
        &self,
        quality_definition_id: i64,
    ) -> Result<QualityDefinition> {
        self.api
            .get_into(Request::new(path_join(&[
                BP_QUALITY_DEFINITION,
                &str_val(quality_definition_id),
            ])))
            .await
    }

    /// Updates a quality definition.
    pub async fn update_quality_definition(
        &self,
        definition: &QualityDefinition,
    ) -> Result<QualityDefinition> {
        let req = Request::new(path_join(&[
            BP_QUALITY_DEFINITION,
            &str_val(definition.id),
        ]))
        .with_json(definition)?;
        self.api.put_into(req).await
    }

    /// Updates all quality definitions.
    pub async fn update_quality_definitions(
        &self,
        definition: &[QualityDefinition],
    ) -> Result<Vec<QualityDefinition>> {
        let req =
            Request::new(path_join(&[BP_QUALITY_DEFINITION, "update"])).with_json(definition)?;
        self.api.put_into(req).await
    }
}
