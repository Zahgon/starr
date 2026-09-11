use super::{Book, Readarr};
use crate::error::Result;
use crate::interface::ApiClientExt;
use crate::is_default;
use crate::paginate::PageReq;
use crate::req::{Request, path_join};
use serde::{Deserialize, Serialize};

const BP_WANTED: &str = "v1/wanted";

/// WantedBooksPage is a paged list of books from `/api/v1/wanted/missing`
/// or `/api/v1/wanted/cutoff`.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WantedBooksPage {
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
    /// Books in this page.
    #[serde(default)]
    pub records: Vec<Book>,
}

impl Readarr {
    /// Returns a page of missing books.
    pub async fn get_wanted_missing_page(
        &self,
        params: Option<&PageReq>,
    ) -> Result<WantedBooksPage> {
        let mut params = params.cloned().unwrap_or_default();
        params.check_set("sortKey", "releaseDate");

        let req = Request::new(path_join(&[BP_WANTED, "missing"])).with_query(params.params());
        self.api.get_into(req).await
    }

    /// Returns a page of books past the quality cutoff.
    pub async fn get_wanted_cutoff_page(
        &self,
        params: Option<&PageReq>,
    ) -> Result<WantedBooksPage> {
        let mut params = params.cloned().unwrap_or_default();
        params.check_set("sortKey", "releaseDate");

        let req = Request::new(path_join(&[BP_WANTED, "cutoff"])).with_query(params.params());
        self.api.get_into(req).await
    }
}
