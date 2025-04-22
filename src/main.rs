mod cli;
mod utils;

use cli::args::GoodboiArgs;
use cli::commands::handle_command;
use clap::{Parser, CommandFactory};
use utils::fs_utils::create_config_folder;

#[tokio::main]
async fn main() -> anyhow::Result<()> {

    create_config_folder().await;

    let args = GoodboiArgs::parse();
    
    if let Some(command) = args.command {
        handle_command(command).await?;
    } else {
        let mut cmd = <GoodboiArgs>::command();
        cmd.print_help()?;
    }
    
    Ok(())
}