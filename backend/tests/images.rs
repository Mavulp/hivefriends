use axum::http::header::AUTHORIZATION;
use serde_json::*;

mod util;
use util::*;

#[tokio::test]
async fn upload() {
    let (client, _temp) = setup_test_client().await;
    let (token, _) = authenticate(&client).await;

    upload_test_image("./tests/testimage.png", &client, &token).await;
}

#[tokio::test]
async fn upload_with_exif() {
    let (client, _temp) = setup_test_client().await;
    let (token, _) = authenticate(&client).await;

    let image_key = upload_test_image("./tests/exif.jpg", &client, &token).await;

    let res = client
        .get(&format!("/api/images/{image_key}"))
        .header(AUTHORIZATION, format!("Bearer {token}"))
        .send()
        .await;

    let status = dbg!(res.status());

    let json = res.json::<Value>().await;
    dbg!(&json);
    assert_eq!(status, 200);

    assert_eq!(json["takenAt"].as_u64().unwrap().to_owned(), 1224692919,);

    let location = json
        .as_object()
        .unwrap()
        .get("location")
        .unwrap()
        .as_object()
        .unwrap();

    assert_eq!(
        location["latitude"].as_str().unwrap().to_owned(),
        "43.46744833333334",
    );

    assert_eq!(
        location["longitude"].as_str().unwrap().to_owned(),
        "11.885126666663888",
    );
}

#[tokio::test]
async fn get_by_key() {
    let (client, _temp) = setup_test_client().await;
    let (token, uploader_key) = authenticate(&client).await;

    let image_key = upload_test_image("./tests/testimage.png", &client, &token).await;

    let res = client
        .get(&format!("/api/images/{image_key}"))
        .header(AUTHORIZATION, format!("Bearer {token}"))
        .send()
        .await;

    let status = dbg!(res.status());

    let json = res.json::<Value>().await;
    dbg!(&json);
    assert_eq!(status, 200);

    assert_eq!(
        json["uploaderKey"].as_str().unwrap().to_owned(),
        uploader_key
    );
}

/* TODO Functionality to list images is still missing
#[tokio::test]
async fn get_all() {
    let (client, _temp) = setup_test_client().await;
    let (token, uploader_key) = authenticate(&client).await;

    let image_key = upload_test_image(&client, &token).await;

    let res = client
        .get(&format!("/api/images/"))
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
        image_key
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

    panic!()
}
*/
