use dirs::config_dir;
use eyre::OptionExt;
use luxnulla::{CONFIG_DIR, SOCKET_NAME, XRAY_CONFIG_FILE};
use std::{fs, path::PathBuf, sync::Arc};
use tokio::net::UnixListener;
mod client_handler;
mod common;
mod handlers;
mod http;
mod services;

#[tokio::main]
async fn main() -> eyre::Result<()> {
    let config_dir_path = config_dir()
        .ok_or_eyre("cannot get a dir")?
        .join(CONFIG_DIR);

    let application = Arc::new(client_handler::ClientHandler::new(config_dir_path.clone()));

    if !config_dir_path.exists() {
        std::fs::create_dir(&config_dir_path)?;
    }

    if !config_dir_path.join(XRAY_CONFIG_FILE).exists() {
        std::fs::File::create(&config_dir_path.join(XRAY_CONFIG_FILE)).unwrap();
    }

    let sock_path = PathBuf::from("/tmp/").join(SOCKET_NAME);
    if sock_path.exists() {
        fs::remove_file(&sock_path)?;
    }

    let listener = UnixListener::bind(&sock_path)?;
    println!("Luxnulla listening on {:?}", sock_path);

    let _ = http::server::init();

    loop {
        let app_clone = application.clone();

        let (sock, _) = listener.accept().await?;

        tokio::spawn(async move { app_clone.handle_client(sock).await });
    }
}
