#![allow(dead_code)]

use axum::http::header::AUTHORIZATION;
use axum_test_helper::TestClient;
use reqwest::multipart::{Form, Part};
use serde_json::{json, Value};
use tempdir::TempDir;

use hivefriends::{
    api_route,
    cli::{run_subcommand, AddUserArgs, SubCommands},
    setup_database,
};

use std::fs::File;
use std::io::Read;

pub async fn setup_test_client() -> (TestClient, TempDir) {
    let temp_dir = TempDir::new("hivefriends-test").unwrap();

    let db_path = temp_dir.path().join("test.db");
    let pool = setup_database(&db_path).await.unwrap();

    let data_path = temp_dir.path().join("data").into();

    let conn = pool.get().await.unwrap();
    let args = AddUserArgs {
        username: String::from("username"),
        password: Some(String::from("password")),
    };
    let sub = SubCommands::AddUser(args);
    conn.interact(move |conn| run_subcommand(sub, conn))
        .await
        .unwrap()
        .unwrap();

    (TestClient::new(api_route(pool, data_path)), temp_dir)
}

pub async fn authenticate(client: &TestClient) -> (String, String) {
    let res = client
        .post("/api/login")
        .json(&json!({"username":"username","password":"invalid"}))
        .send()
        .await;
    assert_eq!(res.status(), 401);

    let res = client
        .post("/api/login")
        .json(&json!({"username":"username","password":"password"}))
        .send()
        .await;
    assert_eq!(res.status(), 200);

    let json = res.json::<Value>().await;
    let token = json["bearerToken"].as_str().unwrap();
    let username = json["username"].as_str().unwrap();

    (token.into(), username.into())
}

pub async fn upload_test_image(path: &str, client: &TestClient, token: &str) -> String {
    let mut file = File::open(path).unwrap();
    let mut data = Vec::new();
    file.read_to_end(&mut data).unwrap();

    let part = Part::bytes(data).file_name("testimage.png");
    let form = Form::new().part("file", part);

    let res = client
        .post("/api/images/")
        .header(AUTHORIZATION, format!("Bearer {token}"))
        .multipart(form)
        .send()
        .await;

    let status = dbg!(res.status());

    let json = res.json::<Value>().await;
    dbg!(&json);
    assert_eq!(status, 200);

    json["key"].as_str().unwrap().into()
}

pub async fn create_test_album(client: &TestClient, token: &str) -> String {
    let image_key = upload_test_image("./tests/testimage.png", &client, &token).await;

    let res = client
        .post(&format!("/api/albums/"))
        .header(AUTHORIZATION, format!("Bearer {token}"))
        .json(&json!({
            "title": "Test Title",
            "description": "Test Description",
            "coverKey": image_key,
            "locations": "home,outside",
            "draft": true,
            "timeframe": {
                "from": 123,
                "to": 234
            },
            "imageKeys": [
                image_key
            ]
        }))
        .send()
        .await;

    let status = dbg!(res.status());

    let json = res.json::<Value>().await;
    dbg!(&json);
    assert_eq!(status, 200);

    json["key"].as_str().unwrap().into()
}
