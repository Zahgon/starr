use super::Radarr;
use crate::error::Result;
use crate::helpers::str_val;
use crate::interface::ApiClientExt;
use crate::is_default;
use crate::req::{Request, path_join};
use crate::shared::Image;
use crate::values::Values;
use serde::{Deserialize, Serialize};

const BP_CREDIT: &str = "v3/credit";

/// Credit is a cast/crew credit from `/api/v3/credit`.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Credit {
    /// Credit ID.
    #[serde(default, skip_serializing_if = "is_default")]
    pub id: i64,
    /// TMDb credit ID.
    #[serde(default, skip_serializing_if = "is_default")]
    pub credit_id: String,
    /// Name of the person.
    #[serde(default, skip_serializing_if = "is_default")]
    pub person_name: String,
    /// Job the person did.
    #[serde(default, skip_serializing_if = "is_default")]
    pub job: String,
    /// Department the person worked in.
    #[serde(default, skip_serializing_if = "is_default")]
    pub department: String,
    /// Character the person played.
    #[serde(default, skip_serializing_if = "is_default")]
    pub character: String,
    /// Order this credit appears in.
    #[serde(default, skip_serializing_if = "is_default")]
    pub order: i32,
    /// Path to the person's profile image.
    #[serde(default, skip_serializing_if = "is_default")]
    pub profile_path: String,
    /// TMDb ID of the person.
    #[serde(default, skip_serializing_if = "is_default", rename = "personTmdbId")]
    pub person_tmdb_id: i64,
    /// Movie this credit belongs to.
    #[serde(default, skip_serializing_if = "is_default")]
    pub movie_id: i64,
    /// Movie metadata this credit belongs to.
    #[serde(default, skip_serializing_if = "is_default")]
    pub movie_metadata_id: i64,
    /// Type of the credit.
    #[serde(default, skip_serializing_if = "is_default", rename = "type")]
    pub credit_type: String,
    /// Images of the person.
    #[serde(default, skip_serializing_if = "is_default")]
    pub images: Vec<Image>,
}

impl Radarr {
    /// Returns credits for a movie or movie metadata.
    pub async fn get_credits(&self, movie_id: i64, movie_metadata_id: i64) -> Result<Vec<Credit>> {
        let mut params = Values::new();
        if movie_id != 0 {
            params.set("movieId", str_val(movie_id));
        }

        if movie_metadata_id != 0 {
            params.set("movieMetadataId", str_val(movie_metadata_id));
        }

        self.api
            .get_into(Request::new(BP_CREDIT).with_query(params))
            .await
    }

    /// Returns a single credit by id.
    pub async fn get_credit(&self, credit_id: i64) -> Result<Credit> {
        self.api
            .get_into(Request::new(path_join(&[BP_CREDIT, &str_val(credit_id)])))
            .await
    }
}
