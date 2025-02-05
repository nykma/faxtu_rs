use serde::{Deserialize, Serialize};
use actix_web::{web, get, post, HttpResponse, Responder};

#[derive(Deserialize)]
struct PingRequest {
    name: String
}

#[derive(Serialize)]
struct PingResponse {
    message: String
}

#[get("/ping")]
pub async fn get_ping() -> impl Responder {
    HttpResponse::Ok().json(PingResponse {
        message: "GET pong".into(),
    })
}

#[post("/ping")]
pub async fn post_ping(req: web::Json<PingRequest>) -> impl Responder {
    HttpResponse::Ok().json(PingResponse {
        message: format!("POST pong, {}", req.name),
    })
}
