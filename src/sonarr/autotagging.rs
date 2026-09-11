use super::Sonarr;
use crate::error::Result;
use crate::helpers::str_val;
use crate::interface::ApiClientExt;
use crate::req::{Request, path_join};

const BP_AUTO_TAGGING: &str = "v3/autotagging";

/// AutoTagging is the `/api/v3/autotagging` resource.
pub type AutoTagging = crate::starrshared::AutoTagging;

/// AutoTaggingSpecification is one rule inside an [`AutoTagging`] definition.
pub type AutoTaggingSpecification = crate::starrshared::AutoTaggingSpecification;

impl Sonarr {
    /// Returns all auto tagging configurations.
    pub async fn get_auto_taggings(&self) -> Result<Vec<AutoTagging>> {
        self.api.get_into(Request::new(BP_AUTO_TAGGING)).await
    }

    /// Returns a single auto tagging configuration.
    pub async fn get_auto_tagging(&self, id: i32) -> Result<AutoTagging> {
        self.api
            .get_into(Request::new(path_join(&[BP_AUTO_TAGGING, &str_val(id)])))
            .await
    }

    /// Returns the specification schema templates for auto tagging.
    pub async fn get_auto_tagging_schema(&self) -> Result<Vec<AutoTaggingSpecification>> {
        self.api
            .get_into(Request::new(path_join(&[BP_AUTO_TAGGING, "schema"])))
            .await
    }

    /// Creates an auto tagging configuration.
    pub async fn add_auto_tagging(&self, input: &AutoTagging) -> Result<AutoTagging> {
        self.api
            .post_into(Request::new(BP_AUTO_TAGGING).with_json(input)?)
            .await
    }

    /// Updates an auto tagging configuration.
    pub async fn update_auto_tagging(&self, input: &AutoTagging) -> Result<AutoTagging> {
        let req =
            Request::new(path_join(&[BP_AUTO_TAGGING, &str_val(input.id)])).with_json(input)?;
        self.api.put_into(req).await
    }

    /// Deletes an auto tagging configuration.
    pub async fn delete_auto_tagging(&self, id: i32) -> Result<()> {
        self.api
            .delete_any(Request::new(path_join(&[BP_AUTO_TAGGING, &str_val(id)])))
            .await
    }
}
