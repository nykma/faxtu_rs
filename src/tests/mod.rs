use actix_web::dev::HttpServiceFactory;
use actix_web::{test, App};
use serde::de::DeserializeOwned;
use serde::Serialize;

mod api;

/// GET JSON-based API in test case.
pub async fn json_api_get<'a, Service, Response>(
    uri: &str,
    service: Service,
) -> Response
where
    Service: HttpServiceFactory + 'static,
    Response: DeserializeOwned,
{
    let app = test::init_service(App::new().service(service)).await;
    let req = test::TestRequest::get().uri(uri).to_request();
    return test::call_and_read_body_json(&app, req).await;
}

/// POST JSON-based API in test case.
pub async fn json_api_post<'a, Service, Body, Response>(
    uri: &str,
    service: Service,
    body: &Body,
) -> Response
where
    Service: HttpServiceFactory + 'static,
    Body: Serialize,
    Response: DeserializeOwned,
{
    let app = test::init_service(App::new().service(service)).await;
    let req = test::TestRequest::post().uri(uri).set_json(body).to_request();
    return test::call_and_read_body_json(&app, req).await;
}
