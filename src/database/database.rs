use crate::{Message, Chat, HashMap};
use aws_sdk_dynamodb::operation::put_item::PutItemInput;
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

    let mut item: HashMap<String, AttributeValue> = HashMap::new();
    item.insert("CompanyName".to_string(), AttributeValue::S("TestCompany".to_string()));
    item.insert("ProjectId_Date".to_string(), AttributeValue::S(sort_key_value.clone()));
    item.insert("MessageId".to_string(), AttributeValue::S(message.message_id.to_string()));

    // Conditionally add TextMessage attribute if message.text is not empty
    if let Some(text) = &message.text {
        if !text.is_empty() {
            item.insert("TextMessage".to_string(), AttributeValue::S(text.clone()));
        }
    }

    // Conditionally add ImagePath attribute if image_path is not empty
    if !image_path.is_empty() {
        item.insert("ImagePath".to_string(), AttributeValue::S(image_path.to_string()));
    }

    // Conditionally add Caption attribute if message.caption is not empty
    if let Some(caption) = &message.caption {
        if !caption.is_empty() {
            item.insert("Caption".to_string(), AttributeValue::S(caption.clone()));
        }
    }

    let input = PutItemInput::builder().table_name("TgChatMessages").set_item(Some(item));

    // Construct the item to be saved in DynamoDB
    let result = input.send_with(&client).await;

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
