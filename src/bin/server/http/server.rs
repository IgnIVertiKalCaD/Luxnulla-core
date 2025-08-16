use axum::Json;
use axum::{Router, routing::get};
use reqwest::Method;
use serde_json::{Value, json};
use tower::ServiceBuilder;
use tower_http::cors::{Any, CorsLayer};

use crate::http;
use crate::services;

const SOCKET: &str = "0.0.0.0:3000";

async fn root() -> &'static str {
    return "Server is working";
}

#[axum::debug_handler]
async fn get_parsed_xray_subs() -> Json<Value> {
    const URL: &str = "https://raw.githubusercontent.com/barry-far/V2ray-Config/refs/heads/main/Splitted-By-Protocol/vless.txt";

    let configs = services::subscription::get_configs(URL).await;

    let configs = configs
        .unwrap()
        .iter()
        .map(|config| http::services::model::xray_config::XrayClientConfig::new(config))
        .collect::<Vec<_>>();

    Json(json!(configs))
}

pub async fn init() {
    let cors_layer = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods([Method::GET, Method::POST]);

    let app: Router<()> = Router::new()
        .route("/", get(root))
        .route("/subs", get(get_parsed_xray_subs))
        .layer(ServiceBuilder::new().layer(cors_layer));

    let listener = tokio::net::TcpListener::bind(SOCKET).await.unwrap();

    println!("http server bind on {}", SOCKET);

    axum::serve(listener, app).await.unwrap();
}
