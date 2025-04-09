use clap::{command, Parser, Subcommand};
use crate::utils::consts::VERSION;

#[derive(Parser,Debug)]
#[command(author="yellowhatpro", version=VERSION, about="Project on steroids", long_about = None)]
pub struct GoodboiArgs {
    #[command(subcommand)]
    pub command: Option<Commands>,
}

#[derive(Debug, Clone)]
pub enum ProjectLanguage {
    Rust,
    Python,
    Typescript,
    Other,
}

impl std::fmt::Display for ProjectLanguage {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ProjectLanguage::Rust => write!(f, "rust"),
            ProjectLanguage::Python => write!(f, "python"),
            ProjectLanguage::Typescript => write!(f, "typescript"),
            ProjectLanguage::Other => write!(f, "other"),
        }
    }
}

#[derive(Debug, Clone)]
pub enum PackageManager {
    Cargo,
    Uv,
    Pnpm,
    Other,
}

impl std::fmt::Display for PackageManager {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PackageManager::Cargo => write!(f, "cargo"),
            PackageManager::Uv => write!(f, "uv"),
            PackageManager::Pnpm => write!(f, "pnpm"),
            PackageManager::Other => write!(f, "other"),
        }
    }
}

#[derive(Subcommand,Debug)]
pub enum Commands {
    /// Initialize a new project with goodboi.toml
    Init,
    /// Start the project (launches TUI with configured services)
    Start,
    /// Install project dependencies
    Install {
        /// Package name to install
        #[arg(short, long)]
        package: Option<String>,
        /// Install as dev dependency
        #[arg(short, long)]
        dev: bool,
    },
    /// Run project commands
    Run {
        /// Command to run (e.g., test, build)
        #[arg(short, long)]
        command: String,
    },
    /// Add a service to the project (db, cache, etc)
    Add {
        /// Service type (postgres, redis, etc)
        #[arg(short, long)]
        service: String,
    },
}