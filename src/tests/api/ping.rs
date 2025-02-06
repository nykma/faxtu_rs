use crate::api::ping::*;
use super::super::{json_api_get, json_api_post};

#[actix_web::test]
async fn test_ping_get() {
    let resp: PingResponse = json_api_get("/ping", get_ping).await;
    assert!(resp.message == "GET pong".to_string());
}

#[actix_web::test]
async fn test_ping_post() {
    let resp: PingResponse = json_api_post("/ping", post_ping, &PingRequest {
        name: "test".to_string(),
    }).await;
    assert!(resp.message == "POST pong, test".to_string());
}
