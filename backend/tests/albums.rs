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
async fn get_by_key() {
    let (client, _temp) = setup_test_client().await;
    let (token, username) = authenticate(&client).await;

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

    assert_eq!(json["key"].as_str().unwrap().to_owned(), album_key);
    assert_eq!(json["author"].as_str().unwrap().to_owned(), username);

    assert_eq!(json["title"].as_str().unwrap().to_owned(), "Test Title");

    assert_eq!(
        json["description"].as_str().unwrap().to_owned(),
        "Test Description"
    );
}

#[tokio::test]
async fn get_all_with_drafts() {
    let (client, _temp) = setup_test_client().await;
    let (token, username) = authenticate(&client).await;

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

    assert_eq!(json["key"].as_str().unwrap().to_owned(), album_key);
    assert_eq!(json["author"].as_str().unwrap().to_owned(), username);
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

#[tokio::test]
async fn share_album() {
    let (client, _temp) = setup_test_client().await;
    let (token, _) = authenticate(&client).await;

    let expected_album_key = create_test_album(&client, &token).await;

    let res = client
        .post(&format!("/api/public/albums/{}", expected_album_key))
        .header(AUTHORIZATION, format!("Bearer {token}"))
        .send()
        .await;

    let status = dbg!(res.status());

    let json = res.json::<Value>().await;
    dbg!(&json);
    assert_eq!(status, 200);

    let token = json["token"].as_str().unwrap();
    let res = client
        .get(&format!("/api/public/albums/{expected_album_key}/{token}"))
        .send()
        .await;

    let status = dbg!(res.status());

    let json = res.json::<Value>().await;
    dbg!(&json);
    assert_eq!(status, 200);

    let actual_album_key = json["key"].as_str().unwrap();
    assert_eq!(actual_album_key, expected_album_key);

    let image_key = json["images"].as_array().unwrap()[0].as_object().unwrap()["key"]
        .as_str()
        .unwrap();
    let res = client
        .get(&format!(
            "/api/public/comments/{expected_album_key}/{image_key}/{token}"
        ))
        .send()
        .await;

    let status = dbg!(res.status());

    let json = res.json::<Value>().await;
    dbg!(&json);
    assert_eq!(status, 200);
}
