use super::Lidarr;
use crate::error::Result;
use crate::helpers::str_val;
use crate::interface::ApiClientExt;
use crate::req::{Request, path_join};

const BP_CUSTOM_FILTER: &str = "v1/customfilter";

/// CustomFilter is the `/api/v1/customfilter` resource.
pub type CustomFilter = crate::starrshared::CustomFilter;

impl Lidarr {
    /// Returns all custom filters.
    pub async fn get_custom_filters(&self) -> Result<Vec<CustomFilter>> {
        self.api.get_into(Request::new(BP_CUSTOM_FILTER)).await
    }

    /// Returns a single custom filter.
    pub async fn get_custom_filter(&self, id: i32) -> Result<CustomFilter> {
        self.api
            .get_into(Request::new(path_join(&[BP_CUSTOM_FILTER, &str_val(id)])))
            .await
    }

    /// Creates a custom filter.
    pub async fn add_custom_filter(&self, input: &CustomFilter) -> Result<CustomFilter> {
        self.api
            .post_into(Request::new(BP_CUSTOM_FILTER).with_json(input)?)
            .await
    }

    /// Updates a custom filter.
    pub async fn update_custom_filter(&self, input: &CustomFilter) -> Result<CustomFilter> {
        let req =
            Request::new(path_join(&[BP_CUSTOM_FILTER, &str_val(input.id)])).with_json(input)?;
        self.api.put_into(req).await
    }

    /// Deletes a custom filter.
    pub async fn delete_custom_filter(&self, id: i32) -> Result<()> {
        self.api
            .delete_any(Request::new(path_join(&[BP_CUSTOM_FILTER, &str_val(id)])))
            .await
    }
}
