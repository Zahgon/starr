use super::{Movie, Radarr};
use crate::error::Result;
use crate::interface::ApiClientExt;
use crate::is_default;
use crate::paginate::PageReq;
use crate::req::{Request, path_join};
use serde::{Deserialize, Serialize};

const BP_WANTED: &str = "v3/wanted";

/// WantedMoviesPage is a paged list of movies from `/api/v3/wanted/missing`
/// or `/api/v3/wanted/cutoff`.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WantedMoviesPage {
    /// Page number of this response.
    #[serde(default)]
    pub page: i32,
    /// Records requested per page.
    #[serde(default)]
    pub page_size: i32,
    /// Key the records are sorted by.
    #[serde(default, skip_serializing_if = "is_default")]
    pub sort_key: String,
    /// Direction the records are sorted in.
    #[serde(default, skip_serializing_if = "is_default")]
    pub sort_direction: String,
    /// Total records available across all pages.
    #[serde(default)]
    pub total_records: i32,
    /// Movies in this page.
    #[serde(default)]
    pub records: Vec<Movie>,
}

impl Radarr {
    /// Returns a page of missing movies.
    pub async fn get_wanted_missing_page(
        &self,
        params: Option<&PageReq>,
    ) -> Result<WantedMoviesPage> {
        let mut params = params.cloned().unwrap_or_default();
        params.check_set("sortKey", "releaseDate");

        let req = Request::new(path_join(&[BP_WANTED, "missing"])).with_query(params.params());
        self.api.get_into(req).await
    }

    /// Returns a page of movies past the quality cutoff.
    pub async fn get_wanted_cutoff_page(
        &self,
        params: Option<&PageReq>,
    ) -> Result<WantedMoviesPage> {
        let mut params = params.cloned().unwrap_or_default();
        params.check_set("sortKey", "releaseDate");

        let req = Request::new(path_join(&[BP_WANTED, "cutoff"])).with_query(params.params());
        self.api.get_into(req).await
    }
}
