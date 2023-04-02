use axum::http::header::AUTHORIZATION;
use serde_json::*;

mod util;
use util::*;

#[tokio::test]
async fn get_by_username() {
    let (client, _temp) = setup_test_client().await;
    let (token, username) = authenticate(&client).await;

    let album_key = create_test_album(&client, &token).await;

    let res = client
        .get(&format!("/api/users/{username}"))
        .header(AUTHORIZATION, format!("Bearer {token}"))
        .send()
        .await;

    let status = dbg!(res.status());

    let json = res.json::<Value>().await;
    dbg!(&json);
    assert_eq!(status, 200);

    let array = json
        .as_object()
        .unwrap()
        .get("albumsUploaded")
        .unwrap()
        .as_array()
        .unwrap();
    assert_eq!(array.len(), 1);
    assert_eq!(array[0], album_key);
}

#[tokio::test]
async fn set_country() {
    let (client, _temp) = setup_test_client().await;
    let (token, username) = authenticate(&client).await;

    let expected_country = "DE";

    let res = client
        .put("/api/settings/")
        .header(AUTHORIZATION, format!("Bearer {token}"))
        .json(&json!({ "country": expected_country }))
        .send()
        .await;

    let status = dbg!(res.status());

    let json = res.json::<Value>().await;
    dbg!(&json);
    assert_eq!(status, 200);

    let res = client
        .get(&format!("/api/users/{username}"))
        .header(AUTHORIZATION, format!("Bearer {token}"))
        .send()
        .await;

    let status = dbg!(res.status());

    let json = res.json::<Value>().await;
    dbg!(&json);
    assert_eq!(status, 200);

    let actual_country = json["country"].as_str().unwrap();
    assert_eq!(expected_country, actual_country);
}
