use starr_rust::prowlarr::Prowlarr;
use starr_rust::starrtest::{BODY_NOT_FOUND, MockData};
use starr_rust::{Config, Tag};
use std::time::Duration;

fn client(url: &str) -> Prowlarr {
    Prowlarr::new(Config::new("apikey123", url, Duration::from_secs(5)))
}

#[tokio::test]
async fn get_system_status_hits_the_right_path() {
    let mock = MockData::new(
        "get system status",
        "GET",
        "/api/v1/system/status",
        r#"{"appName":"Prowlarr","version":"1.2.3","isProduction":true,"migrationVersion":33}"#,
    );
    let server = mock.start().await;

    let status = client(&server.url).get_system_status().await.unwrap();

    server.assert_request(&mock);
    assert_eq!(status.app_name, "Prowlarr");
    assert_eq!(status.version, "1.2.3");
    assert_eq!(status.migration_version, 33);
    assert!(status.is_production);
}

#[tokio::test]
async fn add_tag_posts_json_body() {
    let mock = MockData::new("add tag", "POST", "/api/v1/tag", r#"{"id":3,"label":"new"}"#)
        .with_request(r#"{"label":"new"}"#);
    let server = mock.start().await;

    let tag = client(&server.url)
        .add_tag(&Tag {
            id: 0,
            label: "new".into(),
        })
        .await
        .unwrap();

    server.assert_request(&mock);
    assert_eq!(tag.id, 3);
    assert_eq!(tag.label, "new");
}

#[tokio::test]
async fn paged_history_encodes_query_string() {
    let mock = MockData::new(
        "history page",
        "GET",
        "/api/v1/history?page=1&pageSize=10&sortDirection=ascending&sortKey=date",
        r#"{"page":1,"pageSize":10,"totalRecords":1,"records":[{"id":9,"eventType":"indexerQuery"}]}"#,
    );
    let server = mock.start().await;

    let page = client(&server.url).get_history_page(None).await.unwrap();

    server.assert_request(&mock);
    assert_eq!(page.total_records, 1);
    assert_eq!(page.records[0].id, 9);
    assert_eq!(page.records[0].event_type, "indexerQuery");
}

#[tokio::test]
async fn non_200_becomes_a_req_error() {
    let mock = MockData::new("not found", "GET", "/api/v1/tag/1", BODY_NOT_FOUND).with_status(404);
    let server = mock.start().await;

    let err = client(&server.url).get_tag(1).await.unwrap_err();

    server.assert_request(&mock);
    assert!(err.is_status_code(404), "expected a 404, got: {err}");
    assert_eq!(
        err.to_string(),
        "api.Get(v1/tag/1): invalid status code, 404 >= 300, NotFound"
    );
}

#[tokio::test]
async fn delete_tag_sends_delete() {
    let mock = MockData::new("delete tag", "DELETE", "/api/v1/tag/7", "");
    let server = mock.start().await;

    client(&server.url).delete_tag(7).await.unwrap();

    server.assert_request(&mock);
}
