use crate::env::app_env;
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
}

#[derive(Default, Debug, Clone, Deserialize)]
pub struct ConfigWeb {
    pub host: Vec<String>,
    pub port: i64,
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
