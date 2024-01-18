# omni-notify
A notification server written in rust.

- 通过邮件接收通知
- 通过[PushDeer](https://github.com/easychen/pushdeer)接收通知
- 通过Telegram接收通知

**不需要使用的功能无需填写对应的配置参数**

## 接口
所有接口均支持GET，POST-FORM，POST-JSON

### email
- `/email`, 参数
    - `title`: 主题
    - `body`: 正文

### push
- `/push`, 参数
    - `title`: 主题
    - `body`: 正文（选填）

### telegram
- `/telegram`, 参数
    - `text`: 消息内容

## Docker部署
```sh
PORT=8080 # 改成你的端口号
# email
EMAIL=xxxxxx@xxxx.com # 改成你的邮箱
SMTP_SERVER=smtp.xxxxxx.com # 改成你的smtp服务器
SMTP_USERNAME=xxxxxx # 改成你的smtp服务器用户名
SMTP_PASSWORD=xxxxxx # 改成你的smtp服务器密码
# push
PUSHDEER_KEY=xxxxxx # 改成你的pushdeer的key
# telegram
TG_BOT_TOKEN=xxxxxx # 改成你的telegram bot token
TG_CHAT_ID=xxxxxx # 改成你的chat id，先向bot发送一条消息，然后向https://api.telegram.org/bot<Bot_Token>/getUpdates发起请求即可获取chat_id
# 部署
docker run -d --name omni-notify -p $PORT:8080 hlf01/omni-notify --email $EMAIL --smtp-server $SMTP_SERVER --smtp-username $SMTP_USERNAME --smtp-password $SMTP_PASSWORD --pushdeer-key $PUSHDEER_KEY --tg-bot-token $TG_BOT_TOKEN --tg-chat-id $TG_CHAT_ID
```

## Docker Compose部署
```yml
version: '3.8'
services:
  omni-notify:
    container_name: omni-notify
    image: hlf01/omni-notify
    restart: always
    ports:
      - xxxx:8080 # 改成你的端口号
    # 以下改成你自己的配置
    command: >
      --email xxxx@xxx.com
      --smtp-server smtp.xxxx.com
      --smtp-username xxxxxx
      --smtp-password xxxxxx
      --pushdeer-key xxxxxx
      --tg-bot-token xxxxxx
      --tg-chat-id xxxxxx
```

## Docker构建
```sh
docker build --tag omni-notify --no-cache .
```

## Rust构建
```sh
cargo build --release
# 运行
./target/release/omni-notify --port $PORT --email $EMAIL --smtp-server $SMTP_SERVER --smtp-username $SMTP_USERNAME --smtp-password $SMTP_PASSWORD --pushdeer-key $PUSHDEER_KEY --tg-bot-token $TG_BOT_TOKEN --tg-chat-id $TG_CHAT_ID
```
