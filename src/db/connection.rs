use axum::extract::State;
use sqlx::{PgPool, Row};
use std::process;

use crate::config::{DbType, Settings};
use crate::db::sql::SqlxRepo;

/// Orchestrates the database connection based on settings.
/// It delegates the actual connection logic to the specific repo (e.g., SqlxRepo).
pub async fn connect(settings: &Settings) -> PgPool {
    match settings.database.db_type {
        DbType::Sql => {
            println!("[*] Connecting to SQL (Postgres) at {}...", settings.database.host);

            // SqlxRepo::new returns Result<Self, anyhow::Error>
            match SqlxRepo::new(&settings.database.url).await {
                Ok(repo) => repo.pool,
                Err(e) => {
                    // We print the anyhow error gracefully
                    eprintln!("[!] Failed to connect to the DB: {e}");
                    process::exit(1);
                }
            }
        }
        DbType::Surreal => {
            eprintln!("[!] SurrealDB is not implemented yet.");
            process::exit(1);
        }
    }
}

/// Axum Handler: Tests if the database connection is alive and queries users.
pub async fn test_database_connection(State(pool): State<PgPool>) -> String {
    let query = "SELECT uuid::TEXT, email FROM users";

    #[allow(clippy::uninlined_format_args)]
    match sqlx::query(query).fetch_all(&pool).await {
        Ok(rows) => {
            if rows.is_empty() {
                return "[+] Database Connected! (Table 'users' is currently empty)".to_string();
            }
            let mut output = String::from("[+] Users in Database:\n");
            for row in rows {
                let uuid: String = row.try_get("uuid").unwrap_or_default();
                let email: String = row.try_get("email").unwrap_or_default();
                output.push_str(&format!("  - ID: {}, Email: {}\n", uuid, email));
            }
            output
        }
        Err(e) => format!("[!] Database Query Failed: {}", e),
    }
}