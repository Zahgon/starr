use super::{Movie, Radarr};
use crate::error::Result;
use crate::interface::ApiClientExt;
use crate::is_default;
use crate::req::Request;
use crate::shared::ApplyTags;
use crate::string_enum;
use serde::{Deserialize, Serialize};

const BP_MOVIE_EDITOR: &str = "v3/movie/editor";

/// BulkEdit is the input for the bulk movie editor endpoint.
///
/// Use `Some(...)` to fill the optional members.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BulkEdit {
    /// Movies to edit.
    #[serde(default)]
    pub movie_ids: Vec<i64>,
    /// Monitored state to apply.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub monitored: Option<bool>,
    /// Quality profile to apply.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub quality_profile_id: Option<i64>,
    /// Minimum availability to apply.
    #[serde(default, skip_serializing_if = "is_default")]
    pub minimum_availability: Availability,
    /// Root folder to move the movies into.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub root_folder_path: Option<String>,
    /// Tags to apply.
    #[serde(default, skip_serializing_if = "is_default")]
    pub tags: Vec<i32>,
    /// How the tags are applied.
    #[serde(default, skip_serializing_if = "is_default")]
    pub apply_tags: ApplyTags,
    /// Whether files move with the movie.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub move_files: Option<bool>,
    /// Whether files are deleted. Delete only.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub delete_files: Option<bool>,
    /// Whether deleted movies are excluded from imports. Delete only.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub add_import_exclusion: Option<bool>,
}

string_enum! {
    /// Availability is an enum used as `minimumAvailability` in a few places
    /// throughout Radarr.
    ///
    /// Schema doc'd here: <https://radarr.video/docs/api/#/MovieEditor/put_api_v3_movie_editor>
    pub struct Availability {
        /// The movie release has not been announced.
        const TO_BE_ANNOUNCED = "tba";
        /// The movie release has been announced.
        const ANNOUNCED = "announced";
        /// The movie is in cinemas.
        const IN_CINEMAS = "inCinemas";
        /// The movie has been released.
        const RELEASED = "released";
        /// The movie was deleted.
        const DELETED = "deleted";
    }
}

impl Radarr {
    /// Allows bulk editing many movies at once.
    pub async fn edit_movies(&self, edit_movies: &BulkEdit) -> Result<Vec<Movie>> {
        let req = Request::new(BP_MOVIE_EDITOR).with_json(edit_movies)?;
        self.api.put_into(req).await
    }

    /// Bulk deletes movies.
    ///
    /// Can also mark them as excluded, and delete their files.
    pub async fn delete_movies(&self, delete_movies: &BulkEdit) -> Result<()> {
        let req = Request::new(BP_MOVIE_EDITOR).with_json(delete_movies)?;
        self.api.delete_any(req).await
    }
}
