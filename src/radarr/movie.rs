use super::{Availability, MovieFile, Radarr};
use crate::error::Result;
use crate::helpers::str_val;
use crate::interface::ApiClientExt;
use crate::is_default;
use crate::req::{Request, path_join};
use crate::shared::{Image, OpenRatings, Value};
use crate::values::Values;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

const BP_MOVIE: &str = "v3/movie";

/// Movie is the `/api/v3/movie` endpoint.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Movie {
    /// Movie ID.
    #[serde(default)]
    pub id: i64,
    /// Movie title.
    #[serde(default, skip_serializing_if = "is_default")]
    pub title: String,
    /// Path to the movie folder.
    #[serde(default, skip_serializing_if = "is_default")]
    pub path: String,
    /// Minimum availability before the movie is searched for.
    #[serde(default, skip_serializing_if = "is_default")]
    pub minimum_availability: Availability,
    /// Quality profile applied to this movie.
    #[serde(default, skip_serializing_if = "is_default")]
    pub quality_profile_id: i64,
    /// TMDb ID.
    #[serde(default, skip_serializing_if = "is_default", rename = "tmdbId")]
    pub tmdb_id: i64,
    /// Original title of the movie.
    #[serde(default, skip_serializing_if = "is_default")]
    pub original_title: String,
    /// Alternative titles for this movie.
    #[serde(default, skip_serializing_if = "is_default")]
    pub alternate_titles: Vec<AlternativeTitle>,
    /// Source of the secondary year.
    #[serde(default, skip_serializing_if = "is_default")]
    pub secondary_year_source_id: i32,
    /// Title used for sorting.
    #[serde(default, skip_serializing_if = "is_default")]
    pub sort_title: String,
    /// Bytes used on disk.
    #[serde(default, skip_serializing_if = "is_default")]
    pub size_on_disk: i64,
    /// Movie status.
    #[serde(default, skip_serializing_if = "is_default")]
    pub status: String,
    /// Movie overview.
    #[serde(default, skip_serializing_if = "is_default")]
    pub overview: String,
    /// When the movie hit cinemas.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub in_cinemas: Option<DateTime<Utc>>,
    /// When the physical media was released.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub physical_release: Option<DateTime<Utc>>,
    /// When the digital media was released.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub digital_release: Option<DateTime<Utc>>,
    /// When the movie was released.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub release_date: Option<DateTime<Utc>>,
    /// Movie images.
    #[serde(default, skip_serializing_if = "is_default")]
    pub images: Vec<Image>,
    /// Movie website.
    #[serde(default, skip_serializing_if = "is_default")]
    pub website: String,
    /// Release year.
    #[serde(default, skip_serializing_if = "is_default")]
    pub year: i32,
    /// YouTube trailer ID.
    #[serde(default, skip_serializing_if = "is_default", rename = "youTubeTrailerId")]
    pub you_tube_trailer_id: String,
    /// Studio that produced the movie.
    #[serde(default, skip_serializing_if = "is_default")]
    pub studio: String,
    /// Name of the movie folder.
    #[serde(default, skip_serializing_if = "is_default")]
    pub folder_name: String,
    /// Runtime in minutes.
    #[serde(default, skip_serializing_if = "is_default")]
    pub runtime: i32,
    /// Title with special characters removed.
    #[serde(default, skip_serializing_if = "is_default")]
    pub clean_title: String,
    /// IMDb ID.
    #[serde(default, skip_serializing_if = "is_default", rename = "imdbId")]
    pub imdb_id: String,
    /// Slug used in URLs.
    #[serde(default, skip_serializing_if = "is_default")]
    pub title_slug: String,
    /// Movie certification.
    #[serde(default, skip_serializing_if = "is_default")]
    pub certification: String,
    /// Genres this movie belongs to.
    #[serde(default, skip_serializing_if = "is_default")]
    pub genres: Vec<String>,
    /// Tags applied to this movie.
    #[serde(default, skip_serializing_if = "is_default")]
    pub tags: Vec<i32>,
    /// When the movie was added.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub added: Option<DateTime<Utc>>,
    /// Ratings from various sources.
    #[serde(default, skip_serializing_if = "is_default")]
    pub ratings: OpenRatings,
    /// The file backing this movie.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub movie_file: Option<MovieFile>,
    /// The collection this movie belongs to.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub collection: Option<MovieCollection>,
    /// Whether a file exists for this movie.
    #[serde(default, skip_serializing_if = "is_default")]
    pub has_file: bool,
    /// Whether the movie is available.
    #[serde(default, skip_serializing_if = "is_default")]
    pub is_available: bool,
    /// Whether the movie is monitored.
    #[serde(default)]
    pub monitored: bool,
    /// Movie popularity.
    #[serde(default)]
    pub popularity: f64,
    /// Original language of the movie.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub original_language: Option<Value>,
    /// Only available upon adding a movie.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub add_options: Option<AddMovieOptions>,
}

/// MovieCollection is the collection summary embedded in a [`Movie`] payload.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MovieCollection {
    /// Collection name.
    #[serde(default)]
    pub name: String,
    /// TMDb ID of the collection.
    #[serde(default, rename = "tmdbId")]
    pub tmdb_id: i64,
    /// Collection images.
    #[serde(default)]
    pub images: Vec<Image>,
}

/// AddMovieInput is the input for a new movie.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AddMovieInput {
    /// Movie title.
    #[serde(default, skip_serializing_if = "is_default")]
    pub title: String,
    /// Slug used in URLs.
    #[serde(default, skip_serializing_if = "is_default")]
    pub title_slug: String,
    /// Minimum availability before the movie is searched for.
    #[serde(default, skip_serializing_if = "is_default")]
    pub minimum_availability: Availability,
    /// Root folder the movie is placed in.
    #[serde(default)]
    pub root_folder_path: String,
    /// TMDb ID.
    #[serde(default, rename = "tmdbId")]
    pub tmdb_id: i64,
    /// Quality profile applied to this movie.
    #[serde(default)]
    pub quality_profile_id: i64,
    /// Legacy profile ID.
    #[serde(default, skip_serializing_if = "is_default")]
    pub profile_id: i64,
    /// Release year.
    #[serde(default, skip_serializing_if = "is_default")]
    pub year: i32,
    /// Movie images.
    #[serde(default, skip_serializing_if = "is_default")]
    pub images: Vec<Image>,
    /// Options applied when adding the movie.
    #[serde(default)]
    pub add_options: Option<AddMovieOptions>,
    /// Tags applied to this movie.
    #[serde(default, skip_serializing_if = "is_default")]
    pub tags: Vec<i32>,
    /// Whether the movie is monitored.
    #[serde(default)]
    pub monitored: bool,
}

/// AddMovieOptions are the options for finding a new movie.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AddMovieOptions {
    /// Whether to search for the movie after adding it.
    #[serde(default)]
    pub search_for_movie: bool,
    /// Allowed values: `movieOnly`, `movieAndCollection`, `none`.
    #[serde(default, skip_serializing_if = "is_default")]
    pub monitor: String,
}

/// AlternativeTitle is part of a [`Movie`].
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AlternativeTitle {
    /// Movie metadata this title belongs to.
    #[serde(default)]
    pub movie_metadata_id: i64,
    /// Movie this title belongs to.
    #[serde(default)]
    pub movie_id: i64,
    /// The alternative title.
    #[serde(default)]
    pub title: String,
    /// Where the title came from.
    #[serde(default)]
    pub source_type: String,
    /// ID of the source.
    #[serde(default)]
    pub source_id: i64,
    /// Number of votes.
    #[serde(default)]
    pub votes: i32,
    /// Number of vote records.
    #[serde(default)]
    pub vote_count: i32,
    /// Language of the title.
    #[serde(default)]
    pub language: Option<Value>,
    /// Alternative title ID.
    #[serde(default)]
    pub id: i64,
}

/// GetMovie represents the input parameters for a movie api request.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct GetMovie {
    /// Set this to retrieve a single movie. Leave it at 0 to retrieve them all.
    pub tmdb_id: i64,
    /// Setting this to true may speed up the response time, but less data is returned.
    pub exclude_local_covers: bool,
}

impl Radarr {
    /// Grabs a movie from the queue, or all movies if `tmdb_id` is 0.
    pub async fn get_movie(&self, get_movie: Option<&GetMovie>) -> Result<Vec<Movie>> {
        let get_movie = get_movie.cloned().unwrap_or_default();

        let mut params = Values::new();
        if get_movie.tmdb_id != 0 {
            params.set("tmdbId", str_val(get_movie.tmdb_id));
        } else {
            // excludeLocalCovers can only be true without a tmdbid.
            params.set(
                "excludeLocalCovers",
                str_val(get_movie.exclude_local_covers),
            );
        }

        self.api
            .get_into(Request::new(BP_MOVIE).with_query(params))
            .await
    }

    /// Grabs a movie from the database by DB [movie] ID.
    pub async fn get_movie_by_id(&self, movie_id: i64) -> Result<Movie> {
        self.api
            .get_into(Request::new(path_join(&[BP_MOVIE, &str_val(movie_id)])))
            .await
    }

    /// Sends a `PUT` request to update a movie in place.
    pub async fn update_movie(
        &self,
        movie_id: i64,
        movie: &Movie,
        move_files: bool,
    ) -> Result<Movie> {
        let mut query = Values::new();
        query.add("moveFiles", str_val(move_files));

        let req = Request::new(path_join(&[BP_MOVIE, &str_val(movie_id)]))
            .with_json(movie)?
            .with_query(query);
        self.api.put_into(req).await
    }

    /// Adds a movie to the queue.
    pub async fn add_movie(&self, movie: &AddMovieInput) -> Result<Movie> {
        self.api
            .post_into(Request::new(BP_MOVIE).with_json(movie)?)
            .await
    }

    /// Searches for movies matching the specified search term.
    pub async fn lookup(&self, term: &str) -> Result<Vec<Movie>> {
        if term.is_empty() {
            return Ok(Vec::new());
        }

        let mut query = Values::new();
        query.set("term", term);

        self.api
            .get_into(Request::new(path_join(&[BP_MOVIE, "lookup"])).with_query(query))
            .await
    }

    /// Returns a movie by its ID.
    pub async fn lookup_id(&self, movie_id: i64) -> Result<Movie> {
        self.lookup_sub(&str_val(movie_id), "", "").await
    }

    /// Searches IMDB for the `imdb_id` provided.
    pub async fn lookup_imdb(&self, imdb_id: &str) -> Result<Movie> {
        self.lookup_sub("imdb", "imdbId", imdb_id).await
    }

    /// Searches TMDB for the `tmdb_id` provided.
    pub async fn lookup_tmdb(&self, tmdb_id: i64) -> Result<Movie> {
        self.lookup_sub("tmdb", "tmdbId", &str_val(tmdb_id)).await
    }

    async fn lookup_sub(&self, sub: &str, name: &str, val: &str) -> Result<Movie> {
        let mut query = Values::new();
        if !name.is_empty() {
            query.set(name, val);
        }

        self.api
            .get_into(Request::new(path_join(&[BP_MOVIE, "lookup", sub])).with_query(query))
            .await
    }

    /// Removes a movie from the database.
    ///
    /// Setting `delete_files` true will delete all content for the movie.
    pub async fn delete_movie(
        &self,
        movie_id: i64,
        delete_files: bool,
        add_import_exclusion: bool,
    ) -> Result<()> {
        let mut query = Values::new();
        query.set("deleteFiles", str_val(delete_files));
        query.set("addImportExclusion", str_val(add_import_exclusion));

        self.api
            .delete_any(Request::new(path_join(&[BP_MOVIE, &str_val(movie_id)])).with_query(query))
            .await
    }

    /// Imports movies from the provided resources.
    pub async fn import_movies(&self, movies: &[Movie]) -> Result<Vec<Movie>> {
        let req = Request::new(path_join(&[BP_MOVIE, "import"])).with_json(movies)?;
        self.api.post_into(req).await
    }
}
