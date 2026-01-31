use aws_sdk_s3::Client;
use aws_sdk_s3::config::Builder;
use std::env;

pub async fn init_s3_client() -> Result<Client, String> {
    // Load AWS credentials from environment
    let credentials = aws_credential_types::Credentials::new(
        env::var("AWS_ACCESS_KEY_ID").unwrap_or_default(),
        env::var("AWS_SECRET_ACCESS_KEY").unwrap_or_default(),
        None,
        None,
        "manual",
    );

    let region = aws_sdk_s3::config::Region::new(
        env::var("AWS_DEFAULT_REGION").unwrap_or_else(|_| "idn".to_string()),
    );

    let mut builder = Builder::new()
        .region(region)
        .credentials_provider(credentials)
        .behavior_version(aws_sdk_s3::config::BehaviorVersion::latest());

    // If custom endpoint is provided (for S3-compatible services)
    if let Ok(endpoint) = env::var("AWS_ENDPOINT") {
        builder = builder.endpoint_url(endpoint);
        
        // Use path style endpoint if specified (for NEO, etc.)
        if env::var("AWS_USE_PATH_STYLE_ENDPOINT")
            .map(|v| v == "true")
            .unwrap_or(false)
        {
            builder = builder.force_path_style(true);
        }
    }

    Ok(Client::from_conf(builder.build()))
}

pub async fn upload_file_to_s3(
    client: &Client,
    bucket: &str,
    key: &str,
    body: Vec<u8>,
) -> Result<String, String> {
    client
        .put_object()
        .bucket(bucket)
        .key(key)
        .body(aws_sdk_s3::primitives::ByteStream::from(body))
        .send()
        .await
        .map_err(|e| format!("Failed to upload to S3: {}", e))?;

    // Generate URL
    let endpoint = env::var("AWS_ENDPOINT").unwrap_or_else(|_| {
        format!(
            "https://{}.s3.amazonaws.com",
            env::var("AWS_BUCKET").unwrap_or_default()
        )
    });

    let url = if env::var("AWS_USE_PATH_STYLE_ENDPOINT")
        .map(|v| v == "true")
        .unwrap_or(false)
    {
        format!("{}/{}/{}", endpoint, bucket, key)
    } else {
        format!("{}/{}/{}", endpoint, bucket, key)
    };

    Ok(url)
}

pub async fn delete_file_from_s3(
    client: &Client,
    bucket: &str,
    key: &str,
) -> Result<(), String> {
    client
        .delete_object()
        .bucket(bucket)
        .key(key)
        .send()
        .await
        .map_err(|e| format!("Failed to delete from S3: {}", e))?;

    Ok(())
}

pub fn generate_s3_key(filename: &str) -> String {
    let timestamp = chrono::Local::now().format("%Y%m%d_%H%M%S").to_string();
    format!("files/{}_{}", timestamp, filename)
}
