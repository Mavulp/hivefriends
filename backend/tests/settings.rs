use axum::http::header::AUTHORIZATION;
use serde_json::*;

mod util;
use util::*;

#[tokio::test]
async fn put_password() {
    let (client, _temp) = setup_test_client().await;
    let (token, username) = authenticate(&client).await;

    let new_password = "not password";

    let res = client
        .put("/api/settings/password")
        .header(AUTHORIZATION, format!("Bearer {token}"))
        .json(&json!({
            "old": "password",
            "new": new_password,
        }))
        .send()
        .await;

    let status = dbg!(res.status());

    let json = res.json::<Value>().await;
    dbg!(&json);
    assert_eq!(status, 200);

    let res = client
        .post("/api/login")
        .json(&json!({
            "username": "username",
            "password": new_password,
        }))
        .send()
        .await;

    let status = dbg!(res.status());

    let json = res.json::<Value>().await;
    dbg!(&json);
    assert_eq!(status, 200);

    assert_eq!(json["username"].as_str().unwrap().to_owned(), username);
}
