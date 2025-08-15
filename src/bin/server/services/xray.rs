use luxnulla::{CommandResponse, ErrorCommandResponse, OkCommandResponse};

pub struct XrayService;

impl XrayService {
    pub fn new() -> Self {
        Self
    }

    pub async fn start_xray(&self, config_path: &str) -> CommandResponse {
        match self.spawn_xray(config_path).await {
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
        &self,
        config_path: &str,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        tokio::process::Command::new("xray")
            .args(&["run", "-c", config_path])
            .spawn()?;
        Ok(())
    }

    pub async fn stop_xray(&self) -> CommandResponse {
        CommandResponse::Ok(OkCommandResponse::Message(String::from(
            "Xray stop command sent",
        )))
    }

    pub async fn restart_xray(&self, config_path: &str) -> CommandResponse {
        self.start_xray(config_path).await
    }
}
