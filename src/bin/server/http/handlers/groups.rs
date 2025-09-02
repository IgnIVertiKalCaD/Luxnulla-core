use crate::{
    common::parsers::proxy_config::{self, ProxyConfig},
    services::{Group, StorageService, xray::fetcher::get_configs},
};
use axum::{Json, extract::State, http::StatusCode, response::IntoResponse};
use base64::{Engine, prelude::BASE64_STANDARD};
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use std::sync::Arc;
use url::Url;

#[derive(Deserialize)]
pub struct CreateGroup {
    name: String,
    config: String,
}

#[derive(Deserialize, Serialize)]
pub struct CreateGroupResponse {
    name: String,
    configs: Value,
}

pub enum ConfigType {
    RAW,
    BASE64,
    URL,
}

fn determine_config_type(config: &str) -> Result<ConfigType, std::io::Error> {
    if proxy_config::is_supported_scheme(config) {
        Ok(ConfigType::RAW)
    } else if config.trim().starts_with("http") || config.trim().starts_with("https") {
        Ok(ConfigType::URL)
    } else if BASE64_STANDARD.decode(config.trim()).is_ok() {
        Ok(ConfigType::BASE64)
    } else {
        Err(std::io::Error::new(
            std::io::ErrorKind::InvalidInput,
            "Invalid config",
        ))
    }
}

async fn process_config(payload: &str) -> Result<Vec<ProxyConfig>, std::io::Error> {
    match determine_config_type(payload)? {
        ConfigType::RAW => {
            if let Ok(_) = Url::parse(&payload) {
                if let Ok(work_result) = proxy_config::work(&payload) {
                    Ok(work_result)
                } else {
                    Err(std::io::Error::new(
                        std::io::ErrorKind::Other,
                        "Failed to perform work with config",
                    ))
                }
            } else {
                Err(std::io::Error::new(
                    std::io::ErrorKind::InvalidInput,
                    "Invalid URL",
                ))
            }
        }
        ConfigType::BASE64 => {
            let raw_config = proxy_config::decode_config_from_base64(payload);

            if let Ok(config) = raw_config {
                if let Ok(_) = Url::parse(&config) {
                    if let Ok(work_result) = proxy_config::work(&config) {
                        Ok(work_result)
                    } else {
                        Err(std::io::Error::new(
                            std::io::ErrorKind::Other,
                            "Failed to perform work with config",
                        ))
                    }
                } else {
                    Err(std::io::Error::new(
                        std::io::ErrorKind::InvalidInput,
                        "Invalid URL",
                    ))
                }
            } else {
                Err(std::io::Error::new(
                    std::io::ErrorKind::InvalidInput,
                    "Invalid config",
                ))
            }
        }
        ConfigType::URL => match get_configs(payload).await {
            Ok(configs) => Ok(configs),
            Err(_) => Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                "Failed to fetch configs",
            )),
        },
    }
}

#[axum::debug_handler]
pub async fn create_group(
    State(storage): State<Arc<StorageService>>,
    Json(payload): Json<CreateGroup>,
) -> impl IntoResponse {
    let decoded_config = match process_config(&payload.config).await {
        Ok(config) => config,
        Err(e) => {
            return (
                StatusCode::BAD_REQUEST,
                Json(json!({
                    "error": "Invalid config format",
                    "details": e.to_string()
                })),
            )
                .into_response();
        }
    };

    let group = Group::new(payload.name.clone(), json!(decoded_config));

    match storage.store_group(group) {
        Ok(()) => (
            StatusCode::CREATED,
            Json(CreateGroupResponse {
                name: payload.name,
                configs: json!(decoded_config),
            }),
        )
            .into_response(),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({
                "error": "Failed to save group",
                "details": e.to_string()
            })),
        )
            .into_response(),
    }
}

#[axum::debug_handler]
pub async fn get_groups(State(storage): State<Arc<StorageService>>) -> impl IntoResponse {
    match storage.get_all_groups() {
        Ok(groups) => (
            StatusCode::OK,
            Json(json!({
                "groups": groups,
                "count": groups.len()
            })),
        )
            .into_response(),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({
                "error": "Failed to retrieve groups",
                "details": e.to_string()
            })),
        )
            .into_response(),
    }
}
