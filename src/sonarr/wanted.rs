use super::{Episode, Sonarr};
use crate::error::Result;
use crate::interface::ApiClientExt;
use crate::is_default;
use crate::paginate::PageReq;
use crate::req::{Request, path_join};
use serde::{Deserialize, Serialize};

const BP_WANTED: &str = "v3/wanted";

/// WantedEpisodesPage is a paged list of episodes from `/api/v3/wanted/missing`
/// or `/api/v3/wanted/cutoff`.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WantedEpisodesPage {
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
    /// Episodes in this page.
    #[serde(default)]
    pub records: Vec<Episode>,
}

impl Sonarr {
    /// Returns a page of missing episodes.
    pub async fn get_wanted_missing_page(
        &self,
        params: Option<&PageReq>,
    ) -> Result<WantedEpisodesPage> {
        let mut params = params.cloned().unwrap_or_default();
        params.check_set("sortKey", "airDateUtc");

        let req = Request::new(path_join(&[BP_WANTED, "missing"])).with_query(params.params());
        self.api.get_into(req).await
    }

    /// Returns a single missing episode by episode ID.
    pub async fn get_wanted_missing_episode(&self, episode_id: i64) -> Result<Episode> {
        self.api
            .get_into(Request::new(path_join(&[
                BP_WANTED,
                "missing",
                &crate::helpers::str_val(episode_id),
            ])))
            .await
    }

    /// Returns a single cutoff-unmet episode by episode ID.
    pub async fn get_wanted_cutoff_episode(&self, episode_id: i64) -> Result<Episode> {
        self.api
            .get_into(Request::new(path_join(&[
                BP_WANTED,
                "cutoff",
                &crate::helpers::str_val(episode_id),
            ])))
            .await
    }

    /// Returns a page of episodes past the quality cutoff.
    pub async fn get_wanted_cutoff_page(
        &self,
        params: Option<&PageReq>,
    ) -> Result<WantedEpisodesPage> {
        let mut params = params.cloned().unwrap_or_default();
        params.check_set("sortKey", "airDateUtc");

        let req = Request::new(path_join(&[BP_WANTED, "cutoff"])).with_query(params.params());
        self.api.get_into(req).await
    }
}
