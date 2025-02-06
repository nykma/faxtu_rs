use core::fmt;
use std::env;

use serde::Deserialize;

#[derive(Default, Clone, Debug, Deserialize)]
pub enum ENV {
    #[default]
    Development,
    Testing,
    Staging,
    Production,
}

impl fmt::Display for ENV {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ENV::Development => write!(f, "development"),
            ENV::Testing => write!(f, "testing"),
            ENV::Staging => write!(f, "staging"),
            ENV::Production => write!(f, "production"),
        }
    }
}

impl From<String> for ENV {
    fn from(env: String) -> Self {
        match env.as_str() {
            "development" => ENV::Development,
            "production" => ENV::Production,
            "testing" => ENV::Testing,
            _ => ENV::Development,
        }
    }
}

/// Fetch and parse runtime ENV.
pub fn app_env() -> ENV {
    if cfg!(test) {
        return ENV::Testing;
    }

    // Set test env if in CI environment
    if env::var("CI") != Err(env::VarError::NotPresent) {
        return ENV::Testing;
    }

    env::var("SERVER_ENV")
        .unwrap_or_else(|_| "development".into())
        .into()
}
