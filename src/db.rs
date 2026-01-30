use mongodb::{Client, options::ClientOptions, Database};
use std::env;
use std::sync::Arc;

#[derive(Clone)]
pub struct AppState {
    pub db: Database,
}

pub async fn init_db() -> Result<Arc<AppState>, mongodb::error::Error> {
    let client_uri = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let options = ClientOptions::parse(client_uri).await?;
    let client = Client::with_options(options)?;
    
    // Database name is already in the connection string usually, but let's be explicit if needed or take from path
    // For simplicity, we assume the connection string includes the db name or we default to a name.
    // The .env has /jaga_sehat_indonesia, so client.default_database() should work if set,
    // otherwise we pick one.
    
    let db = client.database("jaga_sehat_indonesia");

    Ok(Arc::new(AppState { db }))
}
