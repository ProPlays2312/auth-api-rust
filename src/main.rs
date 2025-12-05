// For code snippet completeness - remove in actual code
#![allow(unused)]
#![allow(clippy::uninlined_format_args)]

#![warn(clippy::pedantic)] // Enable Clippy lints
#![allow(clippy::missing_errors_doc)] // Allow missing error docs for brevity

mod config;
mod db;
mod error;
mod models;
mod routes;
mod utils;

use crate::routes::create_router;
use sqlx::Row;
use std::process;
use tokio::net::TcpListener;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

use crate::config::Settings;
use crate::db::connect;

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
    let db_pool = connect(&settings).await;

    // 4. Define Routes
    let app = create_router(db_pool);

    // 5. Start the Server
    let addr = settings.app.address();
    match TcpListener::bind(&addr).await {
        Ok(listener) => {
            println!("[+] Server listening on http://{addr}");
            if let Err(e) = axum::serve(listener, app)
                .with_graceful_shutdown(utils::extras::shutdown_signal())
                .await {
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

