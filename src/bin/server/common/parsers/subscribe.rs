use base64::{Engine as _, engine::general_purpose};
use std::error::Error;

pub async fn fetch(url: &str) -> Result<String, Box<dyn Error + Send + Sync>> {
    let response = reqwest::get(url).await?;

    if !response.status().is_success() {
        return Err(format!("Request failed with status: {}", response.status()).into());
    }

    let body = response.text().await?;

    Ok(body)
}

pub async fn parse(payload: &str) -> Result<String, Box<dyn Error + Send + Sync>> {
    let body = payload.trim();

    let content = match general_purpose::STANDARD.decode(body) {
        Ok(decoded_bytes) => {
            println!("INFO: Content detected as Base64. Decoding...");
            String::from_utf8(decoded_bytes)?
        }
        Err(_) => {
            println!("INFO: Content detected as plain text.");
            body.to_string()
        }
    };

    Ok(content)
}
