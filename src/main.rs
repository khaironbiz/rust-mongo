mod db;
mod models;
mod handlers;
mod docs;
mod routes;
mod validation;
mod s3;
mod repository;
mod services;
mod response;
mod pagination;
mod dto;
mod middleware;

use dotenvy::dotenv;
use std::env;
use std::net::SocketAddr;
use tower_http::trace::TraceLayer;

#[tokio::main]
async fn main() {
    dotenv().ok();
    
    // Initialize tracing
    tracing_subscriber::fmt::init();

    // Connect to database
    let state = match db::init_db().await {
        Ok(s) => s,
        Err(e) => {
            eprintln!("Failed to connect to database: {}", e);
            return;
        }
    };

    // Build router
    let app = routes::create_router(state)
        .layer(TraceLayer::new_for_http());

    let port = env::var("PORT").unwrap_or_else(|_| "8000".to_string());
    let addr = format!("0.0.0.0:{}", port);
    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();

    println!("Server running on http://{}", addr);

    axum::serve(listener, app).await.unwrap();
}
