use crate::{
    common::parsers::proxy_config::ProxyConfig,
    http::services::model::xray_config::{
        Settings, StreamSettings, XrayClientConfig,
    },
};

impl XrayClientConfig {
    pub fn new(config: &ProxyConfig) -> Self {
        XrayClientConfig {
            protocol: config.protocol().to_string(),
            name_client: config.name().map(|name| name.to_string()),
            settings: Settings {
                address: config.address().to_string(),
                port: config.port(),
                users: vec![config.user().unwrap().clone()],
            },
            stream: StreamSettings {
                reality: config.reality_settings().cloned(),
                network: config.network().map(|network| network.to_string()),
                security: config.security().map(|security| security.to_string()),
            },
        }
    }
}
