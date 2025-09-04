use dirs::config_dir;
use luxnulla::{CONFIG_DIR, XRAY_CONFIG_FILE};
use std::sync::Mutex;
use tokio::process::{Child, Command};

static XRAY_CHILD: Mutex<Option<Child>> = Mutex::new(None);

pub async fn start_xray() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    spawn_xray().await?;
    Ok(())
}

pub fn get_xray_status() -> bool {
    let mut child_guard = XRAY_CHILD.lock().unwrap();

    if let Some(child) = child_guard.as_mut() {
        if let Ok(Some(status)) = child.try_wait() {
            *child_guard = None;
            println!("Terminated with status: {}", status);
            false
        } else {
            println!("Running");
            true
        }
    } else {
        println!("Not running");
        false
    }
}

async fn spawn_xray() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let config_path = config_dir()
        .ok_or_else(|| "Failed to get config directory")?
        .join(CONFIG_DIR)
        .join(XRAY_CONFIG_FILE);

    println!("{:?}", config_path.as_os_str().to_str().unwrap_or(""));

    let child = Command::new("xray")
        .args(&["run", "-c", config_path.to_str().ok_or("Invalid path")?])
        .spawn()?;

    *XRAY_CHILD.lock().unwrap() = Some(child);
    Ok(())
}

pub async fn stop_xray() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let child_to_kill = {
        let mut child_guard = XRAY_CHILD.lock().unwrap();
        child_guard.take()
    };

    if let Some(mut child) = child_to_kill {
        if let Err(e) = child.kill().await {
            return Err(e.into());
        }
        println!("Xray stopped successfully.");
        Ok(())
    } else {
        println!("Xray is not running.");
        Err(Box::new(std::io::Error::new(
            std::io::ErrorKind::NotFound,
            "Xray is not running",
        )))
    }
}

pub async fn restart_xray() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    stop_xray().await?;
    start_xray().await?;
    println!("Xray restarted successfully.");
    Ok(())
}
