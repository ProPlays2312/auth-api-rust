#![allow(unused)]
mod config;
mod db;
mod error;
mod models;
mod routes;
mod utils;

use axum::{extract::State, routing::get, Router};
use sqlx::{PgPool, Row};
use tokio::net::TcpListener;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
use dotenvy::dotenv;

use crate::config::{Config, DbType};
use crate::db::sql::SqlxRepo;

// TODO: Soft exit ctrl-c handler

#[tokio::main]
async fn main() {
    dotenv().ok();

    // 1. Initialize Logging
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new(
            std::env::var("RUST_LOG").unwrap_or_else(|_| "debug".into()),
        ))
        .with(tracing_subscriber::fmt::layer())
        .init();

    println!("🚀 Starting Auth API...");

    // 2. Load Configuration
    let config = Config::init();

    // 3. Connect to Database based on Config
    // This 'db_pool' will be passed to our routes via .with_state()
    let db_pool = match config.db_type {
        DbType::Sql => {
            println!("🔌 Connecting to SQL (Postgres)...");
            let repo = SqlxRepo::new(&config.database_url).await
                .expect("Failed to connect to Postgres");
            repo.pool
        }
        DbType::Surreal => {
            // Panic for now, as requested, until we build the Surreal side
            panic!("SurrealDB support is coming next! Change DB_TYPE to 'sql' in .env for now.");
        }
    };

    // 4. Define Routes
    // We pass 'db_pool' into the app state so handlers can use it
    let app = Router::new()
        .route("/", get(test_database_connection)) // <--- CHANGED: Uses our new function
        .with_state(db_pool);

    // 5. Start the Server
    let listener = TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!("✅ Server listening on http://0.0.0.0:3000");

    axum::serve(listener, app).await.unwrap();
}

// --- The Simple Function You Requested ---
// This handler borrows the PgPool from the app state and runs a query.
async fn test_database_connection(State(pool): State<PgPool>) -> String {
    // simple query to fetch uuid and email
    // We cast uuid to TEXT to make it easy to display
    let query = "SELECT uuid::TEXT, email FROM users";

    match sqlx::query(query).fetch_all(&pool).await {
        Ok(rows) => {
            if rows.is_empty() {
                return "✅ Database Connected! (Table 'users' is currently empty)".to_string();
            }

            let mut output = String::from("✅ Users in Database:\n");
            for row in rows {
                let uuid: String = row.get("uuid");
                let email: String = row.get("email");
                output.push_str(&format!("- ID: {}, Email: {}\n", uuid, email));
            }
            output
        }
        Err(e) => format!("❌ Database Query Failed: {}", e),
    }
}