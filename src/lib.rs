pub mod db;
pub mod models;
pub mod handlers;
pub mod routes;
pub mod docs;
pub mod validation;
pub mod s3;
pub mod repository;
pub mod services;
pub mod response;
pub mod pagination;
pub mod dto;
pub mod middleware;

// Re-export AppState for tests and external usage
pub use db::AppState;
// Re-export AuthUser for use in handlers
pub use middleware::AuthUser;

