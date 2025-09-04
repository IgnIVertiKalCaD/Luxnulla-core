use luxnulla::{CommandResponse, ErrorCommandResponse, OkCommandResponse};

pub async fn start_xray(config_path: &str) -> CommandResponse {
    match spawn_xray(config_path).await {
        Ok(_) => {
            CommandResponse::Ok(OkCommandResponse::Message(String::from("xray is started")))
        }
        Err(e) => CommandResponse::Err(ErrorCommandResponse::Message(format!(
            "Failed to start Xray: {}",
            e
        ))),
    }
}

async fn spawn_xray(
    config_path: &str,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    tokio::process::Command::new("xray")
        .args(&["run", "-c", config_path])
        .spawn()?;
    Ok(())
}

pub async fn stop_xray() -> CommandResponse {
    CommandResponse::Ok(OkCommandResponse::Message(String::from(
        "Xray stop command sent",
    )))
}

pub async fn restart_xray(config_path: &str) -> CommandResponse {
    start_xray(config_path).await
}
