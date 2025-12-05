// Fetch configuration settings from environment variables.

/*
List of environment variables:
APP__HOST=
APP__PORT=
APP__DEBUG_LEVEL=
APP__DATABASE_URL=
APP__DATABASE_POOL=
APP__DATABASE_TYPE=
APP__JWT_SECRET=
 */

use config::{Config, ConfigError, Environment};
use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)] // Added Clone for easier usage if needed
#[serde(rename_all = "lowercase")] // Allows "sql" or "surreal" in config/env
pub enum DbType {
    Sql,
    Surreal,
}

#[derive(Debug, Deserialize)]
pub struct Database {
    pub url: String,
    pub pool: u32,
    #[serde(rename = "type")]
    pub db_type: DbType, // <--- ADDED: Missing field
    #[serde(skip, default)]
    pub host: String,
}
#[derive(Debug, Deserialize)]
pub struct AppConfig {
    pub host: String,
    pub port: u16,
    pub debug_level: String,
}
impl AppConfig {
    pub fn verify_debug_level(&self) -> bool {
        matches!(
            self.debug_level.as_str(),
            "error" | "warn" | "info" | "debug" | "trace"
        )
    }
    pub fn address(&self) -> String {
        format!("{}:{}", self.host, self.port)
    }
}
#[derive(Debug, Deserialize)]
pub struct Settings {
    pub database: Database,
    #[serde(flatten)]
    pub app: AppConfig,
}

impl Settings {
    pub fn new() -> Result<Self, ConfigError> {
        let config = Config::builder()
            // App Defaults
            .set_default("host", "127.0.0.1")?
            .set_default("port", 8080)?
            .set_default("debug_level", "info")?
            // Database Defaults
            .set_default("database.url", "postgres://user:pass@localhost:5432/db")?
            .set_default("database.pool", 10)?
            .set_default("database.type", "sql")? // Default to SQL
            .add_source(
                Environment::with_prefix("APP")
                    .separator("__")
                    .try_parsing(true)
            )
            .build()?;

        let mut settings: Settings = config.try_deserialize()?;

        // 2. MANUALLY populate the host field derived from the URL
        settings.database.host = get_host(&settings.database.url);

        Ok(settings)
    }
}


fn get_host(url: &str) -> String {
    // 1. Get the part after the credentials (after '@')
    //    or after the protocol (after '://') if no password is used.
    let host = if let Some(at_index) = url.rfind('@') {
        &url[at_index + 1..]
    } else if let Some(proto_end) = url.find("://") {
        &url[proto_end + 3..]
    } else {
        url
    };

    // 2. Stop at the first '/' found in that part (start of the DB name)
    if let Some(slash_index) = host.find('/') {
        return host[..slash_index].to_string();
    }

    // 3. Fallback: If no '/' exists, return the whole thing
    host.to_string()
}