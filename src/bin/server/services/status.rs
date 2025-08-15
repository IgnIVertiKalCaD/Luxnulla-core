use luxnulla::{CommandResponse, OkCommandResponse};

pub struct StatusService;

impl StatusService {
    pub fn new() -> Self {
        Self
    }

    pub fn get_status(&self) -> CommandResponse {
        CommandResponse::Ok(OkCommandResponse::Message(
            "Luxnulla-core is running".to_string(),
        ))
    }
}
