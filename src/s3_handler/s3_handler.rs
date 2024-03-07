use aws_config::load_defaults;
use aws_config::BehaviorVersion;
use aws_sdk_s3 as s3;
use std::error::Error;
use std::path::Path;

use crate::config::Config;
lazy_static::lazy_static! {
    static ref CONFIG: Config = Config::load();
}

pub async fn upload_to_s3(
    group_name: &str,
    file_id: &str,
    file_path: &str,
) -> Result<String, Box<dyn Error>> {
    // Specify your AWS region
    let my_config = load_defaults(BehaviorVersion::latest()).await;

    // Specify the S3 object key (file name) using the file_id
    let object_key = format!("{}/{}.jpg", group_name, file_id);

    // Initialize the AWS SDK S3 client
    let client = s3::Client::new(&my_config);

    let file_content = Path::new(&file_path);

    // Read the file into a buffer
    let body = s3::primitives::ByteStream::from_path(&file_content).await?;

    let result = client
        .put_object()
        .bucket(&CONFIG.s3_bucket_name)
        .key(object_key.clone())
        .body(body)
        .send()
        .await;

    match result {
        Ok(_) => {
            println!("File uploaded to S3 successfully.");
            Ok(object_key)
        }
        Err(err) => {
            println!("Failed to create S3 object. Error: {:?}", err);
            Err(err.into())
        }
    }
}




