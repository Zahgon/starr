use starr_rust::starrconnect::{
    EventType, RadarrHandler, SonarrHandler, WebhookResponse, parse_radarr, parse_sonarr,
};
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;

const SONARR_GRAB: &str = r#"{
    "eventType": "Grab",
    "instanceName": "Sonarr",
    "applicationUrl": "https://sonarr.example.com",
    "series": {"id": 4, "title": "Star Trek", "tvdbId": 999, "malIds": [1, 2]},
    "episodes": [{"id": 11, "seasonNumber": 2, "episodeNumber": 1, "title": "Pilot",
                  "airDateUtc": "2022-01-26T02:00:00Z"}],
    "release": {"releaseTitle": "Star.Trek.S02E01", "indexer": "Indexer (Prowlarr)",
                "size": 885369406, "customFormatScore": 25,
                "languages": [{"id": 1, "name": "English"}]},
    "downloadClient": "NZBGet",
    "downloadId": "a87bda3c",
    "customFormatInfo": {"customFormats": [{"id": 3, "name": "x264"}], "customFormatScore": 25}
}"#;

const SONARR_IMPORT_COMPLETE: &str = r#"{
    "eventType": "Download",
    "series": {"id": 4, "title": "Star Trek"},
    "episodeFiles": [{"id": 21, "relativePath": "S02/E01.mkv", "size": 100},
                     {"id": 22, "relativePath": "S02/E02.mkv", "size": 200}],
    "fileCount": 2,
    "sourcePath": "/downloads/st",
    "destinationPath": "/tv/Star Trek"
}"#;

const SONARR_SINGLE_DOWNLOAD: &str = r#"{
    "eventType": "Download",
    "series": {"id": 4, "title": "Star Trek"},
    "episodeFile": {"id": 21, "relativePath": "S02/E01.mkv", "size": 100},
    "isUpgrade": true
}"#;

#[test]
fn sonarr_grab_decodes_nested_payload() {
    let event = parse_sonarr(SONARR_GRAB.as_bytes()).unwrap();
    assert_eq!(*event.event_type(), EventType::GRAB);

    let grab = event.get_grab().unwrap();
    assert_eq!(grab.base.instance_name, "Sonarr");
    assert_eq!(grab.series.as_ref().unwrap().tvdb_id, 999);
    assert_eq!(grab.series.as_ref().unwrap().mal_ids, vec![1, 2]);
    assert_eq!(grab.episodes[0].title, "Pilot");
    assert_eq!(grab.release.as_ref().unwrap().size, 885_369_406);
    assert_eq!(
        grab.release.as_ref().unwrap().languages[0].name,
        "English"
    );
    assert_eq!(grab.custom_format_info.unwrap().custom_format_score, 25);
}

#[test]
fn sonarr_download_splits_single_and_batch_imports() {
    let single = parse_sonarr(SONARR_SINGLE_DOWNLOAD.as_bytes()).unwrap();
    assert!(single.get_download().is_ok());
    assert!(
        single.get_import_complete().is_err(),
        "single-file body must not decode as import-complete"
    );

    let batch = parse_sonarr(SONARR_IMPORT_COMPLETE.as_bytes()).unwrap();
    assert!(
        batch.get_download().is_err(),
        "batch body must not decode as a single import"
    );

    let complete = batch.get_import_complete().unwrap();
    assert_eq!(complete.file_count, 2);
    assert_eq!(complete.episode_files.len(), 2);
    assert_eq!(complete.destination_path, "/tv/Star Trek");
}

#[test]
fn sonarr_handler_dispatches_to_the_matching_callback() {
    let grabs = Arc::new(AtomicUsize::new(0));
    let imports = Arc::new(AtomicUsize::new(0));

    let counted = Arc::clone(&grabs);
    let counted_imports = Arc::clone(&imports);

    let handler = SonarrHandler {
        on_grab: Some(Box::new(move |grab| {
            assert_eq!(grab.download_client, "NZBGet");
            counted.fetch_add(1, Ordering::SeqCst);
            Ok(())
        })),
        on_import_complete: Some(Box::new(move |complete| {
            assert_eq!(complete.file_count, 2);
            counted_imports.fetch_add(1, Ordering::SeqCst);
            Ok(())
        })),
        ..Default::default()
    };

    assert_eq!(
        handler.handle("POST", SONARR_GRAB.as_bytes()),
        WebhookResponse::ok()
    );
    assert_eq!(
        handler.handle("POST", SONARR_IMPORT_COMPLETE.as_bytes()),
        WebhookResponse::ok()
    );

    assert_eq!(grabs.load(Ordering::SeqCst), 1);
    assert_eq!(imports.load(Ordering::SeqCst), 1);
}

#[test]
fn sonarr_handler_rejects_bad_requests() {
    let seen = Arc::new(AtomicUsize::new(0));
    let counted = Arc::clone(&seen);

    let handler = SonarrHandler {
        on_error: Some(Box::new(move |_err| {
            counted.fetch_add(1, Ordering::SeqCst);
        })),
        ..Default::default()
    };

    assert_eq!(
        handler.handle("GET", SONARR_GRAB.as_bytes()),
        WebhookResponse::error(405, "method not allowed")
    );
    assert_eq!(
        handler.handle("POST", b"{not json"),
        WebhookResponse::error(400, "invalid json")
    );
    assert_eq!(
        handler.handle("POST", br#"{"eventType":"Nope"}"#),
        WebhookResponse::error(500, "handler error")
    );

    assert_eq!(
        seen.load(Ordering::SeqCst),
        2,
        "on_error runs for the json and unknown-event failures"
    );
}

#[test]
fn radarr_handler_reports_callback_failures() {
    let handler = RadarrHandler {
        on_download: Some(Box::new(|_download| {
            Err("callback blew up".to_string().into())
        })),
        ..Default::default()
    };

    let body = br#"{"eventType":"Download","movie":{"id":7,"title":"Alien"},"isUpgrade":false}"#;

    let event = parse_radarr(body).unwrap();
    assert_eq!(event.get_download().unwrap().movie.unwrap().title, "Alien");

    assert_eq!(
        handler.handle("POST", body),
        WebhookResponse::error(500, "handler error")
    );
}
