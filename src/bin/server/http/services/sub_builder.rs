use serde::{Deserialize, Serialize};

use crate::common::parsers::xray::ProxyConfig;

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    #[serde(skip_serializing_if = "Option::is_none")]
    encryption: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Settings {
    address: String,
    port: u16,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Subscribe {
    protocol: String,
    settings: Settings,
    users: Vec<User>,
}

impl Subscribe {
    pub fn new(config: &ProxyConfig) -> Self {
        Subscribe {
            protocol: config.protocol().to_string(),
            settings: Settings {
                address: config.address().to_string(),
                port: config.port(),
            },
            users: vec![User {
                encryption: config.encryption().map(|enc| enc.to_string()),
                id: config.id().map(|id| id.to_string()),
            }],
        }
    }
}
