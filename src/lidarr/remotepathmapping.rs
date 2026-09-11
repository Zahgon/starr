use super::Lidarr;
use crate::error::Result;
use crate::helpers::str_val;
use crate::interface::ApiClientExt;
use crate::req::{Request, path_join};
use crate::shared::RemotePathMapping;

const BP_REMOTE_PATH_MAPPING: &str = "v1/remotePathMapping";

impl Lidarr {
    /// Returns all configured remote path mappings.
    pub async fn get_remote_path_mappings(&self) -> Result<Vec<RemotePathMapping>> {
        self.api
            .get_into(Request::new(BP_REMOTE_PATH_MAPPING))
            .await
    }

    /// Returns a single remote path mapping.
    pub async fn get_remote_path_mapping(&self, mapping_id: i64) -> Result<RemotePathMapping> {
        self.api
            .get_into(Request::new(path_join(&[
                BP_REMOTE_PATH_MAPPING,
                &str_val(mapping_id),
            ])))
            .await
    }

    /// Creates a remote path mapping.
    pub async fn add_remote_path_mapping(
        &self,
        mapping: &RemotePathMapping,
    ) -> Result<RemotePathMapping> {
        self.api
            .post_into(Request::new(BP_REMOTE_PATH_MAPPING).with_json(mapping)?)
            .await
    }

    /// Updates the remote path mapping.
    pub async fn update_remote_path_mapping(
        &self,
        mapping: &RemotePathMapping,
    ) -> Result<RemotePathMapping> {
        let req = Request::new(path_join(&[BP_REMOTE_PATH_MAPPING, &str_val(mapping.id)]))
            .with_json(mapping)?;
        self.api.put_into(req).await
    }

    /// Removes a single remote path mapping.
    pub async fn delete_remote_path_mapping(&self, mapping_id: i64) -> Result<()> {
        self.api
            .delete_any(Request::new(path_join(&[
                BP_REMOTE_PATH_MAPPING,
                &str_val(mapping_id),
            ])))
            .await
    }
}
