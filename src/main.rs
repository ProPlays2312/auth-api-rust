#![allow(unused)] // For code snippet completeness - remove in actual code
#![warn(clippy::pedantic)] // Enable Clippy lints
#![allow(clippy::missing_errors_doc)] // Allow missing error docs for brevity

mod config;
mod db;
mod error;
mod models;
mod routes;
mod utils;

use axum::{extract::State, routing::get, Router};
use sqlx::{PgPool, Row};
use std::process;
use tokio::net::TcpListener;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

use crate::config::{DbType, Settings};
use crate::db::sql::SqlxRepo;

#[tokio::main]
async fn main() {
    // 1. Initialize Logging
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new(
            std::env::var("RUST_LOG").unwrap_or_else(|_| "info".into()),
        ))
        .with(tracing_subscriber::fmt::layer())
        .init();

    println!("[+] Starting Auth API...");

    // 2. Load Configuration
    let settings = match Settings::new() {
        Ok(s) => s,
        Err(e) => {
            eprintln!("[!] Configuration Error: {e}");
            process::exit(1);
        }
    };

    // 3. Connect to Database
    let db_pool = match settings.database.db_type {
        DbType::Sql => {
            // let host = settings.database.host;
            println!("[*] Connecting to SQL (Postgres) at {}...", settings.database.host);

            match SqlxRepo::new(&settings.database.url).await {
                Ok(repo) => repo.pool,
                Err(e) => {
                    eprintln!("[!] Failed to connect to the DB: {e}");
                    process::exit(1);
                }
            }
        }
        DbType::Surreal => {
            eprintln!("[!] SurrealDB is not implemented yet.");
            process::exit(1);
        }
    };

    // 4. Define Routes
    let app = Router::new()
        .route("/", get(test_database_connection))
        .with_state(db_pool);

    // 5. Start the Server
    let addr = settings.app.address();
    match TcpListener::bind(&addr).await {
        Ok(listener) => {
            println!("[+] Server listening on http://{addr}");
            if let Err(e) = axum::serve(listener, app).await {
                eprintln!("[!] Server crashed: {e}");
                process::exit(1);
            }
        }
        Err(e) => {
            eprintln!("[!] Failed to bind to address {}: {}", addr, e);
            process::exit(1);
        }
    }
}

async fn test_database_connection(State(pool): State<PgPool>) -> String {
    let query = "SELECT uuid::TEXT, email FROM users";

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