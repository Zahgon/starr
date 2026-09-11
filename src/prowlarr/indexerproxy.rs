use super::Prowlarr;
use crate::error::Result;
use crate::helpers::{force_save, str_val};
use crate::interface::ApiClientExt;
use crate::is_default;
use crate::req::{Request, path_join};
use crate::shared::{FieldInput, FieldOutput};
use crate::values::Values;
use serde::{Deserialize, Serialize};

const BP_INDEXER_PROXY: &str = "v1/indexerproxy";

/// IndexerProxyInput is used to create or update an indexer proxy.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct IndexerProxyInput {
    /// Proxy ID. Leave zero when creating.
    #[serde(default, skip_serializing_if = "is_default")]
    pub id: i64,
    /// Proxy name.
    #[serde(default, skip_serializing_if = "is_default")]
    pub name: String,
    /// Implementation type of the proxy.
    #[serde(default, skip_serializing_if = "is_default")]
    pub implementation: String,
    /// Configuration contract name.
    #[serde(default, skip_serializing_if = "is_default")]
    pub config_contract: String,
    /// Implementation-specific settings.
    #[serde(default, skip_serializing_if = "is_default")]
    pub fields: Vec<FieldInput>,
}

/// IndexerProxyOutput is returned from indexer proxy endpoints.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct IndexerProxyOutput {
    /// Proxy ID.
    #[serde(default, skip_serializing_if = "is_default")]
    pub id: i64,
    /// Proxy name.
    #[serde(default, skip_serializing_if = "is_default")]
    pub name: String,
    /// Implementation type of the proxy.
    #[serde(default, skip_serializing_if = "is_default")]
    pub implementation: String,
    /// Display name of the implementation.
    #[serde(default, skip_serializing_if = "is_default")]
    pub implementation_name: String,
    /// Configuration contract name.
    #[serde(default, skip_serializing_if = "is_default")]
    pub config_contract: String,
    /// Implementation-specific settings.
    #[serde(default, skip_serializing_if = "is_default")]
    pub fields: Vec<FieldOutput>,
}

impl Prowlarr {
    /// Returns all indexer proxies.
    pub async fn get_indexer_proxies(&self) -> Result<Vec<IndexerProxyOutput>> {
        self.api.get_into(Request::new(BP_INDEXER_PROXY)).await
    }

    /// Returns a single indexer proxy.
    pub async fn get_indexer_proxy(&self, id: i64) -> Result<IndexerProxyOutput> {
        self.api
            .get_into(Request::new(path_join(&[BP_INDEXER_PROXY, &str_val(id)])))
            .await
    }

    /// Returns indexer proxy templates.
    pub async fn get_indexer_proxy_schema(&self) -> Result<Vec<IndexerProxyOutput>> {
        self.api
            .get_into(Request::new(path_join(&[BP_INDEXER_PROXY, "schema"])))
            .await
    }

    /// Creates an indexer proxy.
    pub async fn add_indexer_proxy(
        &self,
        proxy: &IndexerProxyInput,
        force: bool,
    ) -> Result<IndexerProxyOutput> {
        let req = Request::new(BP_INDEXER_PROXY)
            .with_json(proxy)?
            .with_query(force_save(force));
        self.api.post_into(req).await
    }

    /// Updates an indexer proxy.
    pub async fn update_indexer_proxy(
        &self,
        proxy: &IndexerProxyInput,
        force: bool,
    ) -> Result<IndexerProxyOutput> {
        let req = Request::new(path_join(&[BP_INDEXER_PROXY, &str_val(proxy.id)]))
            .with_json(proxy)?
            .with_query(force_save(force));
        self.api.put_into(req).await
    }

    /// Removes an indexer proxy.
    pub async fn delete_indexer_proxy(&self, id: i64) -> Result<()> {
        self.api
            .delete_any(Request::new(path_join(&[BP_INDEXER_PROXY, &str_val(id)])))
            .await
    }

    /// Tests indexer proxy settings.
    pub async fn test_indexer_proxy(
        &self,
        proxy: &IndexerProxyInput,
        force_test: bool,
    ) -> Result<()> {
        let mut query = Values::new();
        query.set("forceTest", str_val(force_test));

        let req = Request::new(path_join(&[BP_INDEXER_PROXY, "test"]))
            .with_json(proxy)?
            .with_query(query);
        self.api.post_any(req).await
    }
}
