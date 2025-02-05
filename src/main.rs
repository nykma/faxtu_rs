use actix_web::{App, HttpServer};

mod api;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Starting server at 127.0.0.1:8100");
    // Print current process's PID
    println!("PID: {}", std::process::id());

    HttpServer::new(|| {
        App::new()
            .service(api::ping::get_ping)
            .service(api::ping::post_ping)
    })
        .bind("[::1]:8100")?
        .bind("127.0.0.1:8100")?
        // graceful shutdown on SIGINT (2)
        // Timeout process will be killed
        .shutdown_timeout(30)
        .run()
        .await
}
