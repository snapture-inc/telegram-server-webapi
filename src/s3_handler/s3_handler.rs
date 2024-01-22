use aws_config::load_defaults;
use aws_config::BehaviorVersion;
use aws_sdk_s3 as s3;
use std::error::Error;
use std::path::Path;

pub async fn upload_to_s3(bucket_name: &str, group_name: &str, file_id: &str, file_path: &str) -> Result<(), Box<dyn Error>> {
    // Specify your AWS region
    let my_config = load_defaults(BehaviorVersion::latest()).await;

    // Specify the S3 object key (file name) using the file_id
    let object_key = format!("{}/{}.jpg", group_name, file_id);

    // Initialize the AWS SDK S3 client
    let client = s3::Client::new(&my_config);

    let file_content = Path::new(&file_path);

    // Read the file into a buffer
    let body = s3::primitives::ByteStream::from_path(&file_content).await?;

    let result = client.put_object()
        .bucket(bucket_name)
        .key(object_key)
        .body(body)
        .send()
        .await;

    match result {
        Ok(_) => {
            println!("File uploaded to S3 successfully.");
        }
        Err(err) => {
            println!("Failed to create S3 object. Error: {:?}", err);
        }
    }

    Ok(())
}
