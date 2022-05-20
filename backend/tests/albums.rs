use axum::http::header::AUTHORIZATION;
use serde_json::*;

mod util;
use util::*;

#[tokio::test]
async fn create() {
    let (client, _temp) = setup_test_client().await;
    let (token, _) = authenticate(&client).await;

    create_test_album(&client, &token).await;
}

#[tokio::test]
async fn get_by_id() {
    let (client, _temp) = setup_test_client().await;
    let (token, uploader_key) = authenticate(&client).await;

    let album_key = create_test_album(&client, &token).await;

    let res = client
        .get(&format!("/api/albums/{album_key}"))
        .header(AUTHORIZATION, format!("Bearer {token}"))
        .send()
        .await;

    let status = dbg!(res.status());

    let json = res.json::<Value>().await;
    dbg!(&json);
    assert_eq!(status, 200);

    assert_eq!(
        json.as_object()
            .unwrap()
            .get("key")
            .unwrap()
            .as_str()
            .unwrap()
            .to_owned(),
        album_key
    );
    assert_eq!(
        json.as_object()
            .unwrap()
            .get("uploaderKey")
            .unwrap()
            .as_str()
            .unwrap()
            .to_owned(),
        uploader_key
    );

    assert_eq!(
        json.as_object()
            .unwrap()
            .get("title")
            .unwrap()
            .as_str()
            .unwrap()
            .to_owned(),
        "Test Title"
    );

    assert_eq!(
        json.as_object()
            .unwrap()
            .get("description")
            .unwrap()
            .as_str()
            .unwrap()
            .to_owned(),
        "Test Description"
    );

    assert_eq!(
        json.as_object()
            .unwrap()
            .get("locations")
            .unwrap()
            .as_str()
            .unwrap()
            .to_owned(),
        "home,outside"
    );
}

#[tokio::test]
async fn get_all_with_drafts() {
    let (client, _temp) = setup_test_client().await;
    let (token, uploader_key) = authenticate(&client).await;

    let album_key = create_test_album(&client, &token).await;

    let res = client
        .get(&format!("/api/albums/?draft=true"))
        .header(AUTHORIZATION, format!("Bearer {token}"))
        .send()
        .await;

    let status = dbg!(res.status());

    let json = res.json::<Value>().await;
    dbg!(&json);
    assert_eq!(status, 200);

    let mut array = json.as_array().unwrap().clone();
    assert_eq!(array.len(), 1);
    let json = array.pop().unwrap();

    assert_eq!(
        json.as_object()
            .unwrap()
            .get("key")
            .unwrap()
            .as_str()
            .unwrap()
            .to_owned(),
        album_key
    );
    assert_eq!(
        json.as_object()
            .unwrap()
            .get("uploaderKey")
            .unwrap()
            .as_str()
            .unwrap()
            .to_owned(),
        uploader_key
    );
}

#[tokio::test]
async fn get_all_without_drafts() {
    let (client, _temp) = setup_test_client().await;
    let (token, _) = authenticate(&client).await;

    create_test_album(&client, &token).await;

    let res = client
        .get(&format!("/api/albums/"))
        .header(AUTHORIZATION, format!("Bearer {token}"))
        .send()
        .await;

    let status = dbg!(res.status());

    let json = res.json::<Value>().await;
    dbg!(&json);
    assert_eq!(status, 200);

    let array = json.as_array().unwrap().clone();
    assert_eq!(array.len(), 0);
}
