use actix_web::{App, HttpServer};
use tracing::level_filters::LevelFilter;
use tracing_subscriber::EnvFilter;

pub mod api;
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
