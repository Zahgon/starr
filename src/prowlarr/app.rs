use super::Prowlarr;
use crate::error::Result;
use crate::helpers::{force_save, str_val};
use crate::interface::ApiClientExt;
use crate::is_default;
use crate::req::{Request, path_join};
use crate::shared::{FieldInput, FieldOutput};
use crate::values::Values;
use serde::{Deserialize, Serialize};

const BP_APPLICATION: &str = "v1/applications";

/// ApplicationInput is used to create or update a connected application.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ApplicationInput {
    /// Application ID. Leave zero when creating.
    #[serde(default, skip_serializing_if = "is_default")]
    pub id: i64,
    /// Application name.
    #[serde(default, skip_serializing_if = "is_default")]
    pub name: String,
    /// How much of the configuration Prowlarr syncs to the app.
    #[serde(default, skip_serializing_if = "is_default")]
    pub sync_level: String,
    /// Implementation type of the application.
    #[serde(default, skip_serializing_if = "is_default")]
    pub implementation: String,
    /// Configuration contract name.
    #[serde(default, skip_serializing_if = "is_default")]
    pub config_contract: String,
    /// App profile applied to this application.
    #[serde(default, skip_serializing_if = "is_default")]
    pub app_profile_id: i64,
    /// Tags applied to this application.
    #[serde(default, skip_serializing_if = "is_default")]
    pub tags: Vec<i32>,
    /// Implementation-specific settings.
    #[serde(default, skip_serializing_if = "is_default")]
    pub fields: Vec<FieldInput>,
}

/// ApplicationOutput is returned from application endpoints.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ApplicationOutput {
    /// Application ID.
    #[serde(default, skip_serializing_if = "is_default")]
    pub id: i64,
    /// Application name.
    #[serde(default, skip_serializing_if = "is_default")]
    pub name: String,
    /// How much of the configuration Prowlarr syncs to the app.
    #[serde(default, skip_serializing_if = "is_default")]
    pub sync_level: String,
    /// Implementation type of the application.
    #[serde(default, skip_serializing_if = "is_default")]
    pub implementation: String,
    /// Display name of the implementation.
    #[serde(default, skip_serializing_if = "is_default")]
    pub implementation_name: String,
    /// Configuration contract name.
    #[serde(default, skip_serializing_if = "is_default")]
    pub config_contract: String,
    /// App profile applied to this application.
    #[serde(default, skip_serializing_if = "is_default")]
    pub app_profile_id: i64,
    /// Tags applied to this application.
    #[serde(default, skip_serializing_if = "is_default")]
    pub tags: Vec<i32>,
    /// Implementation-specific settings.
    #[serde(default, skip_serializing_if = "is_default")]
    pub fields: Vec<FieldOutput>,
}

impl Prowlarr {
    /// Returns all connected applications.
    pub async fn get_applications(&self) -> Result<Vec<ApplicationOutput>> {
        self.api.get_into(Request::new(BP_APPLICATION)).await
    }

    /// Returns a single application.
    pub async fn get_application(&self, id: i64) -> Result<ApplicationOutput> {
        self.api
            .get_into(Request::new(path_join(&[BP_APPLICATION, &str_val(id)])))
            .await
    }

    /// Creates a connected application.
    pub async fn add_application(
        &self,
        app: &ApplicationInput,
        force: bool,
    ) -> Result<ApplicationOutput> {
        let req = Request::new(BP_APPLICATION)
            .with_json(app)?
            .with_query(force_save(force));
        self.api.post_into(req).await
    }

    /// Updates a connected application.
    pub async fn update_application(
        &self,
        app: &ApplicationInput,
        force: bool,
    ) -> Result<ApplicationOutput> {
        let req = Request::new(path_join(&[BP_APPLICATION, &str_val(app.id)]))
            .with_json(app)?
            .with_query(force_save(force));
        self.api.put_into(req).await
    }

    /// Removes a connected application.
    pub async fn delete_application(&self, id: i64) -> Result<()> {
        self.api
            .delete_any(Request::new(path_join(&[BP_APPLICATION, &str_val(id)])))
            .await
    }

    /// Tests connection settings for an application definition.
    pub async fn test_application(&self, app: &ApplicationInput, force_test: bool) -> Result<()> {
        let mut query = Values::new();
        query.set("forceTest", str_val(force_test));

        let req = Request::new(path_join(&[BP_APPLICATION, "test"]))
            .with_json(app)?
            .with_query(query);
        self.api.post_any(req).await
    }
}
