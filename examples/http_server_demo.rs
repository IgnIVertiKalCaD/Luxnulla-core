use axum::{
    Router,
    extract::State,
    http::StatusCode,
    response::Json,
    routing::{get, post},
};
use chrono;
use serde_json::json;
use std::sync::Arc;
use tempfile::TempDir;
use tokio::net::TcpListener;

// Import the handlers and services from the main binary
#[path = "../src/bin/server/services/mod.rs"]
mod services;

#[path = "../src/bin/server/http/handlers/groups.rs"]
mod groups_handlers;

use groups_handlers::{create_group, get_groups};
use services::{ConfigDecoder, Group, StorageService};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🚀 Starting HTTP Server Demo for Groups API");
    println!("============================================\n");

    // Create a temporary directory for storing groups
    let temp_dir = TempDir::new()?;
    let groups_dir = temp_dir.path();
    println!("📁 Groups will be stored in: {:?}", groups_dir);

    // Initialize storage service
    let storage = Arc::new(
        StorageService::with_directory(groups_dir)
            .map_err(|e| format!("Failed to initialize storage: {}", e))?,
    );
    println!("✅ Storage service initialized");

    // Build the router
    let app = Router::new()
        .route("/groups", post(create_group))
        .route("/groups", get(get_groups))
        .route("/health", get(health_check))
        .route("/demo", get(demo_endpoint))
        .with_state(storage.clone());

    println!("🔧 Routes configured:");
    println!("  POST /groups  - Create a new group");
    println!("  GET  /groups  - List all groups");
    println!("  GET  /health  - Health check");
    println!("  GET  /demo    - Demo instructions");

    // Start the server
    let listener = TcpListener::bind("127.0.0.1:3000").await?;
    let addr = listener.local_addr()?;

    println!("\n🌐 Server running at: http://{}", addr);
    println!("\n📖 Try these examples:");
    println!("curl -X POST http://{}/groups \\", addr);
    println!("  -H 'Content-Type: application/json' \\");
    println!(
        "  -d '{{\"name\": \"test-group\", \"config\": \"{{foo: \\\"bar\\\", port: 8080}}\"}}'"
    );
    println!();
    println!("curl http://{}/groups", addr);
    println!();
    println!("Press Ctrl+C to stop the server");

    // Gracefully handle shutdown
    let server = axum::serve(listener, app);

    tokio::select! {
        result = server => {
            if let Err(err) = result {
                eprintln!("Server error: {}", err);
            }
        }
        _ = tokio::signal::ctrl_c() => {
            println!("\n👋 Shutting down server...");
        }
    }

    println!("✅ Server stopped");
    Ok(())
}

async fn health_check() -> Json<serde_json::Value> {
    Json(json!({
        "status": "healthy",
        "service": "groups-api-demo",
        "timestamp": chrono::Utc::now().to_rfc3339()
    }))
}

async fn demo_endpoint() -> Json<serde_json::Value> {
    Json(json!({
        "title": "Groups API Demo",
        "description": "This demo server showcases the config decoder functionality",
        "endpoints": {
            "POST /groups": {
                "description": "Create a new group with decoded config",
                "example_body": {
                    "name": "my-group",
                    "config": "{foo: \"bar\", port: 8080}"
                },
                "supported_config_formats": [
                    "Regular JSON: {\"key\": \"value\"}",
                    "Unquoted keys JSON: {key: \"value\"}",
                    "Base64 encoded JSON",
                    "URL to fetch JSON from"
                ]
            },
            "GET /groups": {
                "description": "List all stored groups"
            }
        },
        "examples": {
            "regular_json": {
                "name": "regular-config",
                "config": "{\"server\": \"example.com\", \"port\": 443}"
            },
            "unquoted_keys": {
                "name": "unquoted-config",
                "config": "{server: \"example.com\", port: 443, enabled: true}"
            },
            "base64_encoded": {
                "name": "base64-config",
                "config": "eyJzZXJ2ZXIiOiAiZXhhbXBsZS5jb20iLCAicG9ydCI6IDQ0M30="
            },
            "complex_config": {
                "name": "complex-config",
                "config": "{\n  server: \"proxy.example.com\",\n  port: 8080,\n  users: [\n    {name: \"user1\", active: true},\n    {name: \"user2\", active: false}\n  ]\n}"
            }
        }
    }))
}
