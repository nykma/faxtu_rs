use crate::env::{app_env, ENV};
use config::Config;
use serde::Deserialize;

const CONFIG_FILE_PATH: &str = "./config/main";
const CONFIG_FILE_PATH_PREFIX: &str = "./config/";

lazy_static::lazy_static! {
    pub static ref C: AppConfig = parse();
}

#[derive(Default, Debug, Clone, Deserialize)]
pub struct AppConfig {
    pub web: ConfigWeb,
    pub db: ConfigDB,
}

#[derive(Default, Debug, Clone, Deserialize)]
pub struct ConfigWeb {
    pub host: Vec<String>,
    pub port: i64,
}

#[derive(Default, Debug, Clone, Deserialize)]
pub struct ConfigDB {
    pub host: String,
    pub port: i64,
    pub user: String,
    pub password: String,
    pub database: String,
}

pub fn parse() -> AppConfig {
    let s = Config::builder()
        // Default
        .add_source(config::File::with_name(CONFIG_FILE_PATH).required(false))
        // app-env-based config
        .add_source(
            config::File::with_name(&format!("{}{}.toml", CONFIG_FILE_PATH_PREFIX, app_env()))
                .required(false),
        )
        // runtime-ENV-based config
        .add_source(
            config::Environment::with_prefix("APP")
                .separator("__")
                .ignore_empty(true),
        )
        .build()
        .expect("Failed to parse config");
    s.try_deserialize().expect("Failed to deserialize config")
}

/// Database connection schema
pub fn db_link() -> String {
    let postgres = format!(
        "postgres://{}:{}@{}:{}/{}",
        C.db.user, C.db.password, C.db.host, C.db.port, C.db.database
    );
    let in_memory_sqlite = "sqlite::memory:".to_string();
    match app_env() {
        ENV::Development => in_memory_sqlite,
        ENV::Testing => in_memory_sqlite,
        ENV::Staging => postgres,
        ENV::Production => postgres,
    }
}
