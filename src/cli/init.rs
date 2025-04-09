use inquire::{Select, Text};
use crate::cli::args::{ProjectLanguage, PackageManager};
use std::path::PathBuf;

pub struct ProjectConfig {
    pub name: String,
    pub language: ProjectLanguage,
    pub package_manager: PackageManager,
    pub description: Option<String>,
}

pub async fn handle_init() -> anyhow::Result<ProjectConfig> {
    println!("🚀 Welcome to goodboi! Let's create a new project.");
    
    // Get current directory name as default project name
    let default_name = PathBuf::from(std::env::current_dir()?.file_name().unwrap())
        .to_string_lossy()
        .to_string();

    // Project name
    let name = Text::new("What is your project named?")
        .with_default(&default_name)
        .with_help_message("This will be used in various places like package.json, Cargo.toml, etc.")
        .prompt()?;

    // Project language
    let languages = vec![
        ProjectLanguage::Rust,
        ProjectLanguage::Python,
        ProjectLanguage::Typescript,
        ProjectLanguage::Other,
    ];
    let language = Select::new("Select a language:", languages)
        .with_help_message("Choose the primary language for your project")
        .prompt()?;

    // Package manager based on language
    let package_manager = match language {
        ProjectLanguage::Rust => PackageManager::Cargo,
        ProjectLanguage::Python => PackageManager::Uv,
        ProjectLanguage::Typescript => PackageManager::Pnpm,
        ProjectLanguage::Other => PackageManager::Other,
    };

    // Optional description
    let description = Text::new("Add a brief description (optional):")
        .with_help_message("Press Enter to skip")
        .prompt()
        .ok();

    Ok(ProjectConfig {
        name,
        language,
        package_manager,
        description,
    })
} 