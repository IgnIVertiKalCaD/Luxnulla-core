use std::error::Error;

pub async fn fetch(url: &str) -> Result<String, Box<dyn Error + Send + Sync>> {
    let response = reqwest::get(url).await?;

    if !response.status().is_success() {
        return Err(format!("Request failed with status: {}", response.status()).into());
    }

    let body = response.text().await?;

    Ok(body)
}
