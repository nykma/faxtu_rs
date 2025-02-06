use serde::{Deserialize, Serialize};
use actix_web::{web, get, post, HttpResponse, Responder};

#[derive(Serialize, Deserialize)]
pub struct PingRequest {
    pub name: String
}

#[derive(Serialize, Deserialize)]
pub struct PingResponse {
    pub message: String
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
