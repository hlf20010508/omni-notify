# omni-notify
A notification server written in rust.

## Introduction
Send notification through
- Email
- [PushDeer](https://github.com/easychen/pushdeer)
- Telegram

## API
All APIs support `GET`, `POST-FORM`, `POST-JSON`.

- Email
  - Path: `/email`
  - Parameters:
    - `title`
    - `body`

- Pushdeer
  - Path: `/pushdeer`
  - Parameters:
    - `title`
    - `body` (optional)

- Telegram
  - Path: `/telegram`
  - Parameter: `text`

## Deploy
To get your Telegram Chat ID, you need to send a message to your bot and visit `https://api.telegram.org/bot<Bot_Token>/getUpdates` to check `result[0].message.chat.id`. Replace `<Bot_Token>` with your bot token.

**You don't need to set the parameters if you don't need the feature.**

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
      --email-address xxxx@xxx.com
      --email-smtp-server smtp.xxxx.com
      --email-smtp-username xxxxxx
      --email-smtp-password xxxxxx
      --pushdeer-key xxxxxx
      --tg-bot-token xxxxxx
      --tg-chat-id xxxxxx
```

## Docker Build
```sh
docker build --tag omni-notify --no-cache .
```
