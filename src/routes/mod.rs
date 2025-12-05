use crate::db::test_database_connection;
use axum::{routing::get, Router};
use sqlx::PgPool;

// Define a function that returns the configured Router
pub fn create_router(pool: PgPool) -> Router {
    Router::new()
        // Register the health check/test route
        .route("/", get(test_database_connection))
        // Attach the database pool to the router state
        .with_state(pool)
}