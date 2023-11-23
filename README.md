# omni-notify
A notification server written in rust.

- 通过邮件接收通知

## 接口
- `/get/notify/email`, 参数
    - `subject`: 主题
    - `body`: 正文
- `/post/json/notify/email`, 接收`json`格式数据，参数
    - `subject`: 主题
    - `body`: 正文
- `/post/form/notify/email`, 接收`form`格式数据，参数
    - `subject`: 主题
    - `body`: 正文

## Docker部署
```sh
PORT=8080 # 改成你的端口号
EMAIL=xxxx@xxx.com # 改成你的邮箱
SMTP_SERVER=smtp.xxxx.com # 改成你的smtp服务器
USER_NAME=xxxxxx # 改成你的smtp服务器用户名
PASSWORD=xxxxxx # 改成你的smtp服务器密码
docker run -d --name omni-notify -p $PORT:8080 hlf01/omni-notify --email $EMAIL --server $SMTP_SERVER --username $USER_NAME --password $PASSWORD
```

## Docker构建
```sh
docker build --tag omni-notify --no-cache .
```

## Rust构建
```sh
cargo build --release
# 运行
./target/release/omni-notify
```
