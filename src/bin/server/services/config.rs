use luxnulla::{
    CommandResponse, EDITOR_NAME, ErrorCommandResponse, LUXNULLA_CONFIG_FILE, OkCommandResponse,
    XRAY_CONFIG_FILE,
};
use std::path::PathBuf;

pub struct ConfigService {
    config_dir: PathBuf,
}

impl ConfigService {
    pub fn new(config_dir: PathBuf) -> Self {
        Self { config_dir }
    }

    pub async fn edit_xray_config(&self) -> CommandResponse {
        match self.spawn_editor(XRAY_CONFIG_FILE).await {
            Ok(_) => CommandResponse::Ok(OkCommandResponse::Message(String::from(
                "zeditor is running",
            ))),
            Err(e) => CommandResponse::Err(ErrorCommandResponse::Message(format!(
                "Failed to open editor: {}",
                e
            ))),
        }
    }

    pub async fn edit_luxnulla_config(&self) -> CommandResponse {
        match self.spawn_editor(LUXNULLA_CONFIG_FILE).await {
            Ok(_) => CommandResponse::Ok(OkCommandResponse::Message(String::from(
                "zeditor is running",
            ))),
            Err(e) => CommandResponse::Err(ErrorCommandResponse::Message(format!(
                "Failed to open editor: {}",
                e
            ))),
        }
    }

    async fn spawn_editor(
        &self,
        config_file: &str,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        tokio::process::Command::new(EDITOR_NAME)
            .arg(&self.config_dir.join(config_file))
            .spawn()?;
        Ok(())
    }

    pub fn get_config_path(&self, config_file: &str) -> Result<String, String> {
        self.config_dir
            .join(config_file)
            .to_str()
            .map(String::from)
            .ok_or_else(|| "Invalid config path".to_string())
    }

    pub fn get_xray_config_path(&self) -> Result<String, String> {
        let config_path_buf = self.config_dir.join(XRAY_CONFIG_FILE);
        match config_path_buf.to_str() {
            Some(s) => Ok(String::from(s)),
            None => Err(String::from("pizda pathbuffer")),
        }
    }

    pub fn get_luxnulla_config_path(&self) -> Result<String, String> {
        self.get_config_path(LUXNULLA_CONFIG_FILE)
    }
}
