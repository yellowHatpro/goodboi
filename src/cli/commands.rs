use anyhow::Result;
use crate::cli::args::Commands;
use crate::cli::init::handle_init;
use crate::utils::fs_utils::create_project_goodboi_toml;

pub async fn handle_command(command: Commands) -> Result<()> {
    match command {
        Commands::Init => {
            // Get project configuration through interactive prompts
            let config = handle_init().await?;
            
            // Create goodboi.toml in current directory
            create_project_goodboi_toml(&config, ".").await?;
            
            println!("✨ Project initialized successfully!");
            Ok(())
        },
        Commands::Start => {
            println!("Starting project...");
            Ok(())
        },
        Commands::Install { package, dev } => {
            println!("Installing package: {:?}, dev: {}", package, dev);
            Ok(())
        },
        Commands::Run { command } => {
            println!("Running command: {}", command);
            Ok(())
        },
        Commands::Add { service } => {
            println!("Adding service: {}", service);
            Ok(())
        },
    }
} 