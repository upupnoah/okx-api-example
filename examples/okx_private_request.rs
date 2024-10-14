use base64::{engine::general_purpose::STANDARD, Engine as _};
use chrono::Utc;
use dotenv::dotenv;
use hmac::{Hmac, Mac};
use reqwest::Client;
use sha2::Sha256;
use std::env;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 从 .env 文件加载环境变量
    dotenv().ok();

    // 获取 API 密钥和密码短语
    let api_key = env::var("OK_ACCESS_KEY")?;
    let secret_key = env::var("OK_ACCESS_SECRET")?;
    let passphrase = env::var("OK_ACCESS_PASSPHRASE")?;

    // 设置请求参数
    let timestamp = Utc::now().format("%Y-%m-%dT%H:%M:%S.%3fZ").to_string();
    let method = "GET";
    let request_path = "/api/v5/account/balance?ccy=USDT";
    let body = ""; // GET 请求的 body 为空

    // 生成预签名字符串
    let prehash_string = format!("{}{}{}{}", timestamp, method, request_path, body);

    // 使用 HMAC-SHA256 进行签名
    let mut mac = Hmac::<Sha256>::new_from_slice(secret_key.as_bytes())?;
    mac.update(prehash_string.as_bytes());
    let sign = STANDARD.encode(mac.finalize().into_bytes());

    // print
    println!("timestamp: {}", timestamp);
    println!("sign: {}", sign);

    // 构建请求客户端并发送请求
    let client = Client::new();
    let response = client
        // .get(format!("https://www.okx.com{}", request_path))
        .get(format!("https://aws.okx.com{}", request_path))
        .header("OK-ACCESS-KEY", api_key)
        .header("OK-ACCESS-SIGN", sign)
        .header("OK-ACCESS-TIMESTAMP", timestamp)
        .header("OK-ACCESS-PASSPHRASE", passphrase)
        .header("Content-Type", "application/json")
        .send()
        .await?;

    // 处理响应
    let response_text = response.text().await?;
    println!("Response: {}", response_text);

    Ok(())
}
