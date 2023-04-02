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
async fn get_all_drafts() {
    let (client, _temp) = setup_test_client().await;
    let (token, _) = authenticate(&client).await;

    let _ = create_test_album(&client, &token).await;

    let res = client
        .get("/api/albums/?draft=true")
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
async fn get_all_without_drafts() {
    let (client, _temp) = setup_test_client().await;
    let (token, username) = authenticate(&client).await;

    let album_key = create_test_album(&client, &token).await;

    let res = client
        .get("/api/albums/")
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

#[tokio::test]
async fn change_images_with_order() {
    let (client, _temp) = setup_test_client().await;
    let (token, expected_username) = authenticate(&client).await;
    let album_key = create_test_album(&client, &token).await;
    let expected_image_key1 = upload_test_image("./tests/testimage.png", &client, &token).await;
    let expected_image_key2 = upload_test_image("./tests/testimage.png", &client, &token).await;
    let expected_descrption = "test PUT description";

    let res = client
        .put(&format!("/api/albums/{album_key}"))
        .header(AUTHORIZATION, format!("Bearer {token}"))
        .json(&json!({
            "description": expected_descrption,
            "imageKeys": [
                expected_image_key1,
                expected_image_key2
            ],
            "taggedUsers": [
                expected_username
            ]
        }))
        .send()
        .await;

    let status = dbg!(res.status());

    let json = res.json::<Value>().await;
    dbg!(&json);
    assert_eq!(status, 200);

    let res = client
        .get(&format!("/api/albums/{album_key}"))
        .header(AUTHORIZATION, format!("Bearer {token}"))
        .send()
        .await;

    let status = dbg!(res.status());

    let json = res.json::<Value>().await;
    dbg!(&json);
    assert_eq!(status, 200);

    let image_key1 = json["images"].as_array().unwrap()[0].as_object().unwrap()["key"]
        .as_str()
        .unwrap();
    let image_key2 = json["images"].as_array().unwrap()[1].as_object().unwrap()["key"]
        .as_str()
        .unwrap();

    assert_eq!(image_key1, expected_image_key1);
    assert_eq!(image_key2, expected_image_key2);

    let username = json["taggedUsers"].as_array().unwrap()[0].as_str().unwrap();

    assert_eq!(username, expected_username);

    let description = json["description"].as_str().unwrap();
    assert_eq!(description, expected_descrption);
}
