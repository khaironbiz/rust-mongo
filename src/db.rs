use mongodb::{Client, options::ClientOptions, Database};
use std::env;
use std::sync::Arc;
use aws_sdk_s3::Client as S3Client;

#[derive(Clone)]
pub struct AppState {
    pub db: Database,
    pub s3_client: Arc<S3Client>,
}

pub async fn init_db() -> Result<Arc<AppState>, Box<dyn std::error::Error>> {
    let client_uri = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let options = ClientOptions::parse(client_uri).await?;
    let client = Client::with_options(options)?;
    
    let db = client.database("jaga_sehat_indonesia");

    // Initialize S3 client
    let s3_client = Arc::new(crate::s3::init_s3_client().await?);

    Ok(Arc::new(AppState { db, s3_client }))
}
