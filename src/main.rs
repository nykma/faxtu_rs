use actix_web::{App, HttpServer};
use tracing::level_filters::LevelFilter;
use tracing_subscriber::EnvFilter;

pub mod api;
pub mod env;
pub mod config;
#[cfg(test)]
mod tests;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Log system
    let log_subscriber = tracing_subscriber::FmtSubscriber::builder()
        .with_env_filter(
            EnvFilter::builder()
                .with_default_directive(LevelFilter::INFO.into())
                .from_env_lossy()
                .add_directive("actix=debug".parse().unwrap())
                .add_directive("tokio=info".parse().unwrap()),
        )
        .finish();
    tracing::subscriber::set_global_default(log_subscriber)
        .expect("Setting default subscriber failed");

    // HTTP server
    let mut server = HttpServer::new(|| {
        App::new()
            .service(api::ping::get_ping)
            .service(api::ping::post_ping)
    });
    for host in &config::C.web.host {
        server = server.bind(format!("{}:{}", host, config::C.web.port)).expect("Fail to start server: cannot recognize config.web");
    };

    // graceful shutdown on SIGINT (2)
    // Timeout process will be killed
    server.shutdown_timeout(30)
        .run()
        .await
}
