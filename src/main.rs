use actix_web::{web, App, HttpResponse, HttpServer, Result};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use tokio::fs::{self, File};
use tokio::io::AsyncWriteExt;

mod config;
use config::Config;
lazy_static::lazy_static! {
    static ref CONFIG: Config = Config::load();
}
mod routes;
use routes::health::health;

mod s3_handler;
use s3_handler::s3_handler::upload_to_s3;

#[derive(Debug, Serialize, Deserialize)]
struct From {
    id: i64,
    is_bot: bool,
    first_name: String,
    username: String,
    language_code: String,
}

#[derive(Debug, Deserialize, Serialize)]
struct PrivateChat {
    id: i64,
    first_name: String,
    username: String,
    #[serde(rename = "type")]
    chat_type: String,
}

#[derive(Debug, Deserialize, Serialize)]
struct SupergroupChat {
    id: i64,
    title: String,
    #[serde(rename = "type")]
    chat_type: String,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(untagged)]
enum Chat {
    Private(PrivateChat),
    Supergroup(SupergroupChat),
}

#[derive(Debug, Serialize, Deserialize)]
struct Photo {
    file_id: String,
    file_unique_id: String,
    file_size: i64,
    width: i64,
    height: i64,
}

#[derive(Debug, Serialize, Deserialize)]
struct Message {
    message_id: i64,
    from: From,
    chat: Chat,
    date: i64,
    text: Option<String>,
    photo: Option<Vec<Photo>>,
    caption: Option<String>,
}
#[derive(Debug, Serialize, Deserialize)]
struct TelegramUpdate {
    update_id: i64,
    message: Message,
}

async fn handle_update(update: web::Json<TelegramUpdate>) -> Result<HttpResponse> {
    // Access the deserialized data
    let telegram_update: TelegramUpdate = update.into_inner();

    // println!("{}", serde_json::to_string(&telegram_update).unwrap());
    println!(
        "-----------------------------New update received, id: {}---------------------------------",
        telegram_update.update_id
    );
    let mut group_name = String::new();
    // Check if the chat is private
    if let Chat::Supergroup(supergroup_chat) = &telegram_update.message.chat {
        group_name = supergroup_chat.title.clone();
    }

    // Check if the message has text
    if let Some(text) = &telegram_update.message.text {
        println!(
            "Received text message: {} from {} in {}",
            text, telegram_update.message.from.username, group_name
        );
    }

    // Check if the message has photos
    if let Some(photos) = &telegram_update.message.photo {
        // Filter photos with widths 450, 600 and 800
        let filtered_photo: Vec<&Photo> = photos
            .iter()
            .filter(|photo| photo.width == 600 || photo.width == 800 || photo.width == 450)
            .collect();

        // Serialize the filtered photos to a JSON string
        let serialized_photo = serde_json::to_string(&filtered_photo).unwrap();
        println!("{}", serialized_photo);

        // Check if the message has a caption
        if let Some(caption) = &telegram_update.message.caption {
            println!("with caption: {}", caption);
        }

        // Download and save each photo
        for photo in filtered_photo {
            if let Some(file_url) = get_photo_url(&photo.file_id).await {
                // Create a folder for the photos
                println!("Downloading file.........");
                download_and_save_photo(file_url, &group_name, &photo.file_id).await;
            }
        }
    }

    // Respond with a simple message
    Ok(HttpResponse::Ok().body("Update received successfully"))
}

async fn get_photo_url(file_id: &str) -> Option<String> {
    let bot_token = &CONFIG.telegram_bot_token;
    let url = format!(
        "https://api.telegram.org/bot{}/getFile?file_id={}",
        bot_token, file_id
    );

    let response = reqwest::get(&url).await.ok()?;
    let result: serde_json::Value = response.json().await.ok()?;
    let file_path = result["result"]["file_path"].as_str()?;
    Some(format!(
        "https://api.telegram.org/file/bot{}/{}",
        bot_token, file_path
    ))
}

async fn download_and_save_photo(file_url: String, group_name: &String, file_id: &String) {
    let response = reqwest::get(&file_url).await;

    match response {
        Ok(response) => {
            let folder_path = format!("photos/{}", group_name);
            fs::create_dir_all(&folder_path).await.unwrap();

            let extension = Some("jpg");
            if let Some(extension) = extension {
                let file_name = format!("{}/{}.{}", folder_path, file_id, extension);

                // Use tokio::fs::File for asynchronous file I/O
                let mut file = File::create(&file_name).await.unwrap();

                let bytes_response = response.bytes().await.unwrap();
                let bytes = bytes_response.as_ref();

                // Use tokio::io::AsyncWriteExt to write asynchronously
                file.write_all(bytes).await.unwrap();

                println!("Downloaded and saved photo as: {}", file_name);

                upload_to_s3(&group_name, &file_id, &file_name)
                    .await
                    .unwrap();
            } else {
                println!("Failed to determine file extension");
            }
        }
        Err(err) => {
            eprintln!("Error downloading photo: {:?}", err);
        }
    }
}

async fn set_telegram_webhook(bot_token: &str, server_url: &str) -> Result<(), reqwest::Error> {
    let url = format!("https://api.telegram.org/bot{}/setWebhook", bot_token);

    let mut form_data = HashMap::new();
    form_data.insert("url", server_url);
    form_data.insert("allowed_updates", "['messages']");

    let client = Client::new();
    let response = client.post(&url).form(&form_data).send().await?;

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
    let bot_token = &CONFIG.telegram_bot_token;

    // This url is where the webhook update is posted to
    // let server_url = "https://<uuid>.ngrok-free.app/";
    let server_url = &CONFIG.server_url;

    // Set up the Telegram webhook
    if let Err(err) = set_telegram_webhook(bot_token, server_url).await {
        eprintln!("Failed to set webhook: {:?}", err);
    }

    // Set up Actix web server
    HttpServer::new(move || {
        App::new()
            .service(health)
            .service(web::resource("/").route(web::post().to(handle_update)))
    })
    .bind(("0.0.0.0", 8000))? // Replace with your desired IP address and port
    .run()
    .await?;

    Ok(())
}
