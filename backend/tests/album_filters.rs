use axum::http::header::AUTHORIZATION;
use serde_json::*;

mod util;
use util::*;

#[tokio::test]
async fn get_all() {
    let (client, _temp) = setup_test_client().await;
    let (token, username) = authenticate(&client).await;

    let _ = create_test_album(&client, &token).await;

    let res = client
        .get(&format!("/api/albums/filters"))
        .header(AUTHORIZATION, format!("Bearer {token}"))
        .send()
        .await;

    let status = dbg!(res.status());

    let json = res.json::<Value>().await;
    dbg!(&json);
    assert_eq!(status, 200);

    let authors = json["authors"].as_array().unwrap();
    assert_eq!(authors[0].as_str().unwrap(), &username);

    let timeframes = json["timeframes"].as_array().unwrap();
    let timeframe = timeframes[0].as_object().unwrap();

    assert_eq!(timeframe["from"].as_i64().unwrap(), 123);
    assert_eq!(timeframe["to"].as_i64().unwrap(), 234);

    assert_eq!(json["hasDrafts"].as_bool().unwrap(), false);
}
