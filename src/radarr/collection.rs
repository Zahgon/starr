use super::{Availability, Movie, Radarr};
use crate::error::Result;
use crate::helpers::str_val;
use crate::interface::ApiClientExt;
use crate::is_default;
use crate::req::{Request, path_join};
use crate::shared::{ApplyTags, Image};
use crate::values::Values;
use serde::{Deserialize, Serialize};

const BP_COLLECTION: &str = "v3/collection";

/// Collection is the `/api/v3/collection` resource.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Collection {
    /// Collection ID.
    #[serde(default, skip_serializing_if = "is_default")]
    pub id: i32,
    /// Collection title.
    #[serde(default, skip_serializing_if = "is_default")]
    pub title: String,
    /// Title used for sorting.
    #[serde(default, skip_serializing_if = "is_default")]
    pub sort_title: String,
    /// TMDb ID of the collection.
    #[serde(default, skip_serializing_if = "is_default", rename = "tmdbId")]
    pub tmdb_id: i64,
    /// Collection images.
    #[serde(default, skip_serializing_if = "is_default")]
    pub images: Vec<Image>,
    /// Collection overview.
    #[serde(default, skip_serializing_if = "is_default")]
    pub overview: String,
    /// Whether the collection is monitored.
    #[serde(default)]
    pub monitored: bool,
    /// Root folder movies from this collection are placed in.
    #[serde(default, skip_serializing_if = "is_default")]
    pub root_folder_path: String,
    /// Quality profile applied to added movies.
    #[serde(default, skip_serializing_if = "is_default")]
    pub quality_profile_id: i64,
    /// Whether added movies are searched for.
    #[serde(default)]
    pub search_on_add: bool,
    /// Minimum availability applied to added movies.
    #[serde(default, skip_serializing_if = "is_default")]
    pub minimum_availability: Availability,
    /// Movies in this collection.
    #[serde(default, skip_serializing_if = "is_default")]
    pub movies: Vec<Movie>,
    /// Number of movies missing from the library.
    #[serde(default, skip_serializing_if = "is_default")]
    pub missing_movies: i32,
    /// Tags applied to this collection.
    #[serde(default, skip_serializing_if = "is_default")]
    pub tags: Vec<i32>,
}

/// CollectionUpdate is the body for `PUT /collection` (bulk update).
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CollectionUpdate {
    /// Collections to update.
    #[serde(default)]
    pub collection_ids: Vec<i32>,
    /// Monitored state to apply.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub monitored: Option<bool>,
    /// Quality profile to apply.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub quality_profile_id: Option<i32>,
    /// Root folder to apply.
    #[serde(default, skip_serializing_if = "is_default")]
    pub root_folder_path: String,
    /// Whether added movies are searched for.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub search_on_add: Option<bool>,
    /// Minimum availability to apply.
    #[serde(default, skip_serializing_if = "is_default")]
    pub minimum_availability: Availability,
    /// Tags to apply.
    #[serde(default, skip_serializing_if = "is_default")]
    pub tags: Vec<i32>,
    /// How the tags are applied.
    #[serde(default, skip_serializing_if = "is_default")]
    pub apply_tags: ApplyTags,
}

impl Radarr {
    /// Returns collections, optionally filtered by TMDb id.
    pub async fn get_collections(&self, tmdb_id: i64) -> Result<Vec<Collection>> {
        let mut params = Values::new();
        if tmdb_id != 0 {
            params.set("tmdbId", str_val(tmdb_id));
        }

        self.api
            .get_into(Request::new(BP_COLLECTION).with_query(params))
            .await
    }

    /// Returns a single collection by database id.
    pub async fn get_collection(&self, collection_id: i64) -> Result<Collection> {
        self.api
            .get_into(Request::new(path_join(&[
                BP_COLLECTION,
                &str_val(collection_id),
            ])))
            .await
    }

    /// Applies a bulk update to collections.
    pub async fn update_collections(&self, update: &CollectionUpdate) -> Result<Vec<Collection>> {
        let req = Request::new(BP_COLLECTION).with_json(update)?;
        self.api.put_into(req).await
    }

    /// Updates a single collection.
    pub async fn update_collection(&self, collection: &Collection) -> Result<Collection> {
        let req = Request::new(path_join(&[
            BP_COLLECTION,
            &str_val(i64::from(collection.id)),
        ]))
        .with_json(collection)?;
        self.api.put_into(req).await
    }
}
