use starr_rust::lidarr::Lidarr;
use starr_rust::radarr::{AddMovieInput, Radarr};
use starr_rust::readarr::Readarr;
use starr_rust::sonarr::{GetEpisode, Sonarr};
use starr_rust::starrtest::MockData;
use starr_rust::{Config, PageReq};
use std::time::Duration;

fn config(url: &str) -> Config {
    Config::new("apikey123", url, Duration::from_secs(5))
}

#[tokio::test]
async fn sonarr_get_series_sends_season_images_param() {
    let mock = MockData::new(
        "get series",
        "GET",
        "/api/v3/series?includeSeasonImages=true&tvdbId=999",
        r#"[{"id":4,"title":"Star Trek","tvdbId":999,"seasons":[{"seasonNumber":1,"monitored":true}]}]"#,
    );
    let server = mock.start().await;

    let series = Sonarr::new(config(&server.url))
        .get_series(999)
        .await
        .unwrap();

    server.assert_request(&mock);
    assert_eq!(series.len(), 1);
    assert_eq!(series[0].title, "Star Trek");
    assert_eq!(series[0].tvdb_id, 999);
    assert_eq!(series[0].seasons[0].season_number, 1);
}

#[tokio::test]
async fn sonarr_episode_filters_become_query_params() {
    let mock = MockData::new(
        "get episodes",
        "GET",
        "/api/v3/episode?episodeIds=11&episodeIds=12&includeImages=true&seasonNumber=2&seriesId=4",
        r#"[{"id":11,"seriesId":4,"seasonNumber":2,"episodeNumber":1,"title":"Pilot","hasFile":true}]"#,
    );
    let server = mock.start().await;

    let episodes = Sonarr::new(config(&server.url))
        .get_series_episodes(&GetEpisode {
            series_id: 4,
            season_number: 2,
            episode_ids: vec![11, 12],
            include_images: true,
            ..Default::default()
        })
        .await
        .unwrap();

    server.assert_request(&mock);
    assert_eq!(episodes[0].title, "Pilot");
    assert!(episodes[0].has_file);
}

#[tokio::test]
async fn radarr_add_movie_posts_camel_case_json() {
    let mock = MockData::new(
        "add movie",
        "POST",
        "/api/v3/movie",
        r#"{"id":7,"title":"Alien","tmdbId":348,"monitored":true,"ratings":{"imdb":{"votes":9,"value":8.5}}}"#,
    )
    .with_request(
        r#"{"title":"Alien","rootFolderPath":"/movies","tmdbId":348,"qualityProfileId":1,"addOptions":{"searchForMovie":true},"monitored":true}"#,
    );
    let server = mock.start().await;

    let movie = Radarr::new(config(&server.url))
        .add_movie(&AddMovieInput {
            title: "Alien".into(),
            root_folder_path: "/movies".into(),
            tmdb_id: 348,
            quality_profile_id: 1,
            monitored: true,
            add_options: Some(starr_rust::radarr::AddMovieOptions {
                search_for_movie: true,
                ..Default::default()
            }),
            ..Default::default()
        })
        .await
        .unwrap();

    server.assert_request(&mock);
    assert_eq!(movie.id, 7);
    assert_eq!(movie.tmdb_id, 348);
    assert_eq!(movie.ratings["imdb"].value, 8.5);
}

#[tokio::test]
async fn radarr_queue_page_sets_sonarr_free_defaults() {
    let mock = MockData::new(
        "queue page",
        "GET",
        "/api/v3/queue?includeUnknownMovieItems=true&page=1&pageSize=10&sortDirection=ascending&sortKey=timeleft",
        r#"{"page":1,"pageSize":10,"totalRecords":1,"records":[{"id":3,"movieId":7,"title":"Alien.1979","protocol":"usenet","size":123.0}]}"#,
    );
    let server = mock.start().await;

    let queue = Radarr::new(config(&server.url))
        .get_queue_page(None)
        .await
        .unwrap();

    server.assert_request(&mock);
    assert_eq!(queue.records[0].movie_id, 7);
    assert_eq!(
        queue.records[0].protocol,
        starr_rust::shared::Protocol::USENET
    );
}

#[tokio::test]
async fn lidarr_wanted_page_honours_caller_sort_key() {
    let mock = MockData::new(
        "wanted missing",
        "GET",
        "/api/v1/wanted/missing?page=2&pageSize=5&sortDirection=descending&sortKey=title",
        r#"{"page":2,"pageSize":5,"totalRecords":40,"records":[{"id":1,"title":"Kind of Blue"}]}"#,
    );
    let server = mock.start().await;

    let params = PageReq::new()
        .page(2)
        .page_size(5)
        .sort_key("title")
        .sort_dir(starr_rust::Sorting::Descend);

    let page = Lidarr::new(config(&server.url))
        .get_wanted_missing_page(Some(&params))
        .await
        .unwrap();

    server.assert_request(&mock);
    assert_eq!(page.total_records, 40);
    assert_eq!(page.records[0].title, "Kind of Blue");
}

#[tokio::test]
async fn readarr_author_lookup_encodes_the_search_term() {
    let mock = MockData::new(
        "author lookup",
        "GET",
        "/api/v1/author/lookup?term=iain+m+banks",
        r#"[{"id":0,"authorName":"Iain M. Banks","foreignAuthorId":"5807106"}]"#,
    );
    let server = mock.start().await;

    let authors = Readarr::new(config(&server.url))
        .lookup_author("iain m banks")
        .await
        .unwrap();

    server.assert_request(&mock);
    assert_eq!(authors[0].author_name, "Iain M. Banks");
    assert_eq!(authors[0].foreign_author_id, "5807106");
}

#[tokio::test]
async fn empty_search_terms_skip_the_request_like_go() {
    let server = MockData::new("unused", "GET", "/unused", "").start().await;

    let books = Readarr::new(config(&server.url)).lookup("").await.unwrap();
    let movies = Radarr::new(config(&server.url)).lookup("").await.unwrap();

    assert!(books.is_empty());
    assert!(movies.is_empty());
    assert!(server.requests().is_empty(), "no request should be sent");
}
