use crate::{Message, Chat};
use aws_sdk_dynamodb::types::AttributeValue;
use aws_sdk_dynamodb::Client;
use aws_config::BehaviorVersion;
use aws_config::load_defaults;
use std::error::Error;
use chrono::DateTime;

pub async fn store_message_in_dynamodb(message: &Message, image_path: &str) -> Result<(), Box<dyn Error>> {
    // Create a DynamoDB client
    let my_config = load_defaults(BehaviorVersion::latest()).await;
    let client = Client::new(&my_config);

    let project_id = match &message.chat {
        Chat::Supergroup(supergroup_chat) => supergroup_chat.id.to_string(),
        _ => "".to_string(), // Handle other chat types if needed
    };


    let mut datetime_utc = String::new();
    if let Some(datetime) = DateTime::from_timestamp(message.date, 0) {     
        datetime_utc = datetime.with_timezone(&chrono_tz::Asia::Singapore).format("%Y-%m-%dT%H:%M:%SZ").to_string();           
    } else {
        println!("Invalid timestamp");
    }
    
    // Construct the composite sort key value
    let sort_key_value: String = format!("{}#{}", project_id, datetime_utc);

    // Construct the item to be saved in DynamoDB
    let result = client
        .put_item()
        .table_name("TgChatMessages")
        .item("CompanyName", AttributeValue::S("TestCompany".to_string()))
        .item("ProjectId_Date", AttributeValue::S(sort_key_value.clone()))
        .item("MessageId", AttributeValue::S(message.message_id.to_string()))
        .item("TextMessage", AttributeValue::S(message.text.clone().unwrap_or_default()))
        .item("ImagePath", AttributeValue::S(image_path.to_string()))
        .item("Caption", AttributeValue::S(message.caption.clone().unwrap_or_default()))
        .send()
        .await;

    match result {
        Ok(_) => {
            println!("Data saved to dynamodb successfully.");
        }
        Err(err) => {
            println!("Failed to save data to dynamodb. Error: {:?}", err);
        }
    }

    Ok(())
}
