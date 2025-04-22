use tokio::fs;
use crate::cli::init::ProjectConfig;
use anyhow::Result;

use  super::consts::{GOODBOI_CONFIG_DIR, GOODBOI_CONFIG_FILE, HOME_DIR, GLOBAL_CONFIG_FILE};

fn get_home_dir() -> String {
    std::env::var(HOME_DIR).unwrap_or_default()
}

async fn create_folder(path: &str) {
    if let Err(e) = tokio::fs::create_dir_all(path).await {
        log::error!("Failed to create project directory: {}", e);
    }
}

pub async fn create_config_folder() -> Result<()> {
    let config_dir = format!("{}/{}", get_home_dir(), GOODBOI_CONFIG_DIR);
    create_folder(&config_dir).await;
    //create file .gooodboi.toml in the config folder, if it doesn't exist
    if !fs::metadata(&format!("{}/{}", config_dir, GLOBAL_CONFIG_FILE)).await.is_ok() {
        let template = include_str!("../../templates/.goodboi.toml");
        fs::write(&format!("{}/{}", config_dir, GLOBAL_CONFIG_FILE), template).await?;
    }

    Ok(())
}

pub async fn create_project_goodboi_toml(config: &ProjectConfig, path: &str) -> Result<()> {
    let template = include_str!("../../templates/base.toml");
    
    let content = template
        .replace("{{project_name}}", &config.name)
        .replace("{{project_description}}", &config.description.as_deref().unwrap_or(""))
        .replace("{{language_type}}", &config.language.to_string())
        .replace("{{package_manager}}", &config.package_manager.to_string());

    fs::write(&format!("{}/{}", path, GOODBOI_CONFIG_FILE), &content).await?;
    
    Ok(())
}
