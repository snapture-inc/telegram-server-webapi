use actix_web::{web, App, HttpServer, Result, HttpResponse};
use serde::{Deserialize, Serialize};
use reqwest::Client;
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize)]
struct From {
    id: i64,
    is_bot: bool,
    first_name: String,
    username: String,
    language_code: String
}

#[derive(Debug, Deserialize, Serialize)]
struct PrivateChat {
    id: i64,
    first_name: String,
    username: String,
    #[serde(rename = "type")]
    chat_type: String
}

#[derive(Debug, Deserialize, Serialize)]
struct SupergroupChat {
    id: i64,
    title: String,
    #[serde(rename = "type")]
    chat_type: String
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(untagged)]
enum Chat {
    Private(PrivateChat),
    Supergroup(SupergroupChat)
}

#[derive(Debug, Serialize, Deserialize)]
struct Photo {
    file_id: String,
    file_unique_id: String,
    file_size: i64,
    width: i64,
    height: i64
}

#[derive(Debug, Serialize, Deserialize)]
struct Message {
    message_id: i64,
    from: From,
    chat: Chat,
    date: i64,
    text: Option<String>,
    photo: Option<Vec<Photo>>,
    caption: Option<String>
}
#[derive(Debug, Serialize, Deserialize)]
struct TelegramUpdate {
    update_id: i64,
    message: Message
}

async fn handle_update(update: web::Json<TelegramUpdate>) -> Result<HttpResponse> {
    // Access the deserialized data
    let telegram_update: TelegramUpdate = update.into_inner();

    // println!("{}", serde_json::to_string(&telegram_update).unwrap());
    println!("-----------------------------New update received, id: {}---------------------------------", telegram_update.update_id);
    let mut group_name = String::new();
    // Check if the chat is private
     if let Chat::Supergroup(supergroup_chat) = &telegram_update.message.chat {
        group_name = supergroup_chat.title.clone();
    }

    // Check if the message has text
    if let Some(text) = &telegram_update.message.text {
        println!("Received text message: {} from {} in {}", text, telegram_update.message.from.username, group_name);
    }

    // Check if the message has photos
    if let Some(photos) = &telegram_update.message.photo {
        // Filter photos with widths 600 and 800
        let filtered_photos: Vec<&Photo> = photos
            .iter()
            .filter(|photo| photo.width == 600 || photo.width == 800)
            .collect();

        // Serialize the filtered photos to a JSON string
        let serialized_photos = serde_json::to_string(&filtered_photos).unwrap();

        // Do something with the serialized photos
        println!("{}", serialized_photos);
        // Check if the message has a caption
        if let Some(caption) = &telegram_update.message.caption {
            println!("with caption: {}", caption);
        }
    }

    // Respond with a simple message
    Ok(HttpResponse::Ok().body("Update received successfully"))
}

async fn set_telegram_webhook(bot_token: &str, ngrok_url: &str) -> Result<(), reqwest::Error> {
    let url = format!(
        "https://api.telegram.org/bot{}/setWebhook",
        bot_token
    );

    let mut form_data = HashMap::new();
    form_data.insert("url", ngrok_url);
    form_data.insert("allowed_updates", "['messages']");

    let client = Client::new();
    let response = client
        .post(&url)
        .form(&form_data)
        .send()
        .await?;

    if response.status().is_success() {
        println!("Webhook set successfully");
        Ok(())
    } else {
        eprintln!("Error setting webhook: {:?}", response);
        Err(response.error_for_status().unwrap_err())
    }
}

#[actix_web::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Bot token can found in telegram @BotFather
    let bot_token = "<YOUR_BOT_TOKEN>";

    // This url is where the webhook update is posted to
    let ngrok_url = "<YOUR_NGROK_URL>";

    // Set up the Telegram webhook
    if let Err(err) = set_telegram_webhook(bot_token, ngrok_url).await {
        eprintln!("Failed to set webhook: {:?}", err);
    }

    // Set up Actix web server
    HttpServer::new(move || {
        App::new().service(web::resource("/").route(web::post().to(handle_update)))
    })
    .bind("127.0.0.1:80")? // Replace with your desired IP address and port
    .run()
    .await?;

    Ok(())
}
