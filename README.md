# Rust Telegram Bot

This Rust application interacts with the Telegram API. Follow the steps below to set up your Telegram bot and run the application locally.

## Getting Started

### 1. Create a Telegram Bot

1. Open the Telegram app and search for "BotFather" or click [this link](https://t.me/botfather) to open a chat with BotFather.

2. Type `/newbot` to create a new bot. Follow the instructions to choose a name and username for your bot.

3. After completing the setup, BotFather will provide you with a token. Keep this token secure, as it's necessary for interacting with the Telegram API.

#### Set Privacy Mode (Optional):

If your bot needs to access messages from groups or channels, you might need to disable privacy mode. Type `/setprivacy` to BotFather and select your bot. Then, choose "Disable" to allow the bot to receive all messages in groups. Finally, promote your bot to an admin in the group.

### 2. Set Up Ngrok for Local Testing

1. Visit the [ngrok website](https://ngrok.com/) and sign up for a free account.

2. Download the ngrok executable for your operating system.

3. In a terminal, navigate to the directory where you extracted ngrok and run the following command to expose your local server to the internet:
   ```bash
   ngrok http 8000
   ```
   Replace 8000 with the port your Rust application is running on.

### 3. Update telegram webhook

1. Replace '<TELEGRAM_BOT_TOKEN>' with the bot token obtained from step 1 and '<SERVER_URL>' with the ngrok url obtained from step 2.

### 4. Run your rust application

1. Start your Rust application locally:
   ```bash
   cargo run
   ```
