use std::sync::Arc;
use axum::Json;
use axum::{Router, routing::{get, post, put, delete}};
use reqwest::Method;
use serde_json::{Value, json};
use tower::ServiceBuilder;
use tower_http::cors::{Any, CorsLayer};

use crate::http::handlers::groups::{create_group, delete_group, get_groups, update_group};
use crate::http::services::model::xray_config::XrayClientConfig;
use crate::services::{self};

const SOCKET: &str = "0.0.0.0:3000";

async fn root() -> &'static str {
    return "Server is working";
}

#[axum::debug_handler]
async fn get_parsed_xray_configs() -> Json<Value> {
    const URL: &str =
        "https://raw.githubusercontent.com/barry-far/V2ray-Config/refs/heads/main/Sub1.txt";

    let configs = services::xray::fetcher::get_configs(URL).await;

    let configs = configs
        .unwrap()
        .iter()
        .map(|config| XrayClientConfig::new(config))
        .collect::<Vec<_>>();

    Json(json!(configs))
}

pub fn init() -> tokio::task::JoinHandle<()> {
    let storage_service_state = Arc::new(services::StorageService::new());

    tokio::spawn(async {
        let cors_layer = CorsLayer::new()
            .allow_origin(Any)
            .allow_methods([Method::GET, Method::POST, Method::PUT, Method::DELETE])
            .allow_headers(Any);

        let app = Router::new()
            .route("/", get(root))
            .route("/configs", get(get_parsed_xray_configs))
            .route("/groups", get(get_groups))
            .route("/group", post(create_group).put(update_group))
            .route("/group/{name}", delete(delete_group))
            .with_state(storage_service_state)
            .layer(ServiceBuilder::new().layer(cors_layer));

        let listener = tokio::net::TcpListener::bind(SOCKET).await.unwrap();

        println!("http server bind on {}", SOCKET);

        axum::serve(listener, app).await.unwrap();
    })
}
