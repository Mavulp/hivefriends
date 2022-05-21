use serde_json::{json, Value};

mod util;
use util::setup_test_client;

#[tokio::test]
async fn invalid_password() {
    let (client, _temp) = setup_test_client().await;

    let res = client
        .post("/api/login")
        .json(&json!({"username":"username","password":"invalid"}))
        .send()
        .await;
    let status = res.status();

    let json = res.json::<Value>().await;
    dbg!(&json);

    assert_eq!(status, 401);
}

#[tokio::test]
async fn authenticate() {
    let (client, _temp) = setup_test_client().await;

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
    let status = res.status();

    let json = res.json::<Value>().await;
    dbg!(&json);
    let token = json["bearerToken"].as_str().unwrap();

    assert_eq!(status, 200);
    assert_eq!(token.len(), 64);
}
