use crate::services::{xray::XrayService, ConfigService, StatusService};
use luxnulla::{CommandRequest, CommandResponse, ErrorCommandResponse, OkCommandResponse};
use std::path::PathBuf;

pub struct CommandHandler {
    status_service: StatusService,
    config_service: ConfigService,
    xray_service: XrayService,
}

impl CommandHandler {
    pub fn new(config_dir: PathBuf) -> Self {
        Self {
            status_service: StatusService::new(),
            config_service: ConfigService::new(config_dir),
            xray_service: XrayService::new(),
        }
    }

    pub async fn handle_command(&self, request: CommandRequest) -> CommandResponse {
        match request {
            CommandRequest::Status => self.status_service.get_status(),

            CommandRequest::Restart => {
                CommandResponse::Ok(OkCommandResponse::Message(String::from(
                    "xray is restarted TODO",
                )))
                // self.subscription_service
                // .refresh_default_subscription()
                // .await
            }

            CommandRequest::EditXray => self.config_service.edit_xray_config().await,

            CommandRequest::EditLuxnulla => self.config_service.edit_luxnulla_config().await,

            CommandRequest::Start => match self.config_service.get_xray_config_path() {
                Ok(config_path) => self.xray_service.start_xray(&config_path).await,
                Err(e) => CommandResponse::Err(ErrorCommandResponse::Message(e)),
            },
        }
    }
}
