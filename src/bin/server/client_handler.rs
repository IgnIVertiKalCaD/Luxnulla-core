use luxnulla::{CommandRequest, CommandResponse, ErrorCommandResponse};
use std::path::PathBuf;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::UnixStream;

use crate::handlers::CommandHandler;

pub struct ClientHandler {
    command_handler: CommandHandler,
}

impl ClientHandler {
    pub fn new(config_dir: PathBuf) -> Self {
        Self {
            command_handler: CommandHandler::new(config_dir),
        }
    }

    pub async fn handle_client(&self, mut sock: UnixStream) {
        let mut buf = vec![0u8; 1024];
        match sock.read(&mut buf).await {
            Ok(n) if n > 0 => {
                let response = match serde_json::from_slice::<CommandRequest>(&buf[..n]) {
                    Ok(request) => self.command_handler.handle_command(request).await,
                    Err(e) => CommandResponse::Err(ErrorCommandResponse::Message(format!(
                        "bad request: {}",
                        e
                    ))),
                };

                if let Ok(output) = serde_json::to_vec(&response) {
                    let _ = sock.write_all(&output).await;
                }
            }
            _ => {}
        }
    }

    pub fn command_handler(&self) -> &CommandHandler {
        &self.command_handler
    }
}
