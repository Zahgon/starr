use super::{AlternativeTitle, Radarr};
use crate::error::Result;
use crate::helpers::str_val;
use crate::interface::ApiClientExt;
use crate::req::{Request, path_join};
use crate::values::Values;

const BP_ALT_TITLE: &str = "v3/alttitle";

impl Radarr {
    /// Returns alternative titles for a movie.
    pub async fn get_alternative_titles(
        &self,
        movie_id: i64,
        movie_metadata_id: i64,
    ) -> Result<Vec<AlternativeTitle>> {
        let mut params = Values::new();
        if movie_id != 0 {
            params.set("movieId", str_val(movie_id));
        }

        if movie_metadata_id != 0 {
            params.set("movieMetadataId", str_val(movie_metadata_id));
        }

        self.api
            .get_into(Request::new(BP_ALT_TITLE).with_query(params))
            .await
    }

    /// Returns a single alternative title by id.
    pub async fn get_alternative_title(&self, alt_title_id: i64) -> Result<AlternativeTitle> {
        self.api
            .get_into(Request::new(path_join(&[
                BP_ALT_TITLE,
                &str_val(alt_title_id),
            ])))
            .await
    }
}
