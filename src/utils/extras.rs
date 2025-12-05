/// Listens for CTRL+C to allow the server to finish active requests before closing.
pub async fn shutdown_signal() {
    tokio::signal::ctrl_c()
        .await
        .expect("failed to install Ctrl+C handler");

    println!("\n[!] Ctrl+C received. Shutting down gracefully...");
}