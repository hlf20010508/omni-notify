# omni-notify
A notification server written in rust.

## Introduction
Receive notification from
- Email
- [PushDeer](https://github.com/easychen/pushdeer)
- Telegram

**You don't need to set the parameters if you don't need the feature.**

## API
All API support `GET`, `POST-FORM`, `POST-JSON`.

### Email
- `/email`
    - `title`
    - `body`

### Push
- `/pushdeer`
    - `title`
    - `body` (optional)

### Telegram
- `/telegram`
    - `text`

## Launch Through Docker
### Note
- To get your Telegram Chat ID, you need to send a message to your bot, then visit `https://api.telegram.org/bot<Bot_Token>/getUpdates` and check `result[0].message.chat.id`. Replace `<Bot_Token>` with your bot token.

### Launch Directly
```sh
PORT=8080
# Email
EMAIL=xxxxxx@xxxx.com
SMTP_SERVER=smtp.xxxxxx.com
SMTP_USERNAME=xxxxxx
SMTP_PASSWORD=xxxxxx
# Push
PUSHDEER_KEY=xxxxxx
# Telegram
TG_BOT_TOKEN=xxxxxx
TG_CHAT_ID=xxxxxx

# Launch
docker run -d --name omni-notify -p $PORT:8080 hlf01/omni-notify --email $EMAIL --smtp-server $SMTP_SERVER --smtp-username $SMTP_USERNAME --smtp-password $SMTP_PASSWORD --pushdeer-key $PUSHDEER_KEY --tg-bot-token $TG_BOT_TOKEN --tg-chat-id $TG_CHAT_ID
```

### Launch through Docker Compose
```yml
version: '3.8'
services:
  omni-notify:
    container_name: omni-notify
    image: hlf01/omni-notify
    restart: always
    ports:
      - xxxx:8080
    command: >
      --email xxxx@xxx.com
      --smtp-server smtp.xxxx.com
      --smtp-username xxxxxx
      --smtp-password xxxxxx
      --pushdeer-key xxxxxx
      --tg-bot-token xxxxxx
      --tg-chat-id xxxxxx
```

## Docker Build
```sh
docker build --tag omni-notify --no-cache .
```

## Rust Build
```sh
cargo build --release

# Launch
./target/release/omni-notify --port $PORT --email $EMAIL --smtp-server $SMTP_SERVER --smtp-username $SMTP_USERNAME --smtp-password $SMTP_PASSWORD --pushdeer-key $PUSHDEER_KEY --tg-bot-token $TG_BOT_TOKEN --tg-chat-id $TG_CHAT_ID
```
