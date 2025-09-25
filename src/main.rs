mod clipboard;
mod commands;
mod store;

use clap::{Parser, Subcommand};

/// 🦀 RustyClip: a simple clipboard history manager
#[derive(Parser)]
#[command(name = "rustyclip")]
#[command(about = "Save, search, and reuse clipboard history")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Add,
    List,
    Get { index: usize },
    Clear,
    Search { query: String },
}

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Add => commands::add()?,
        Commands::List => commands::list()?,
        Commands::Get { index } => commands::get(index)?,
        Commands::Clear => commands::clear()?,
        Commands::Search { query } => commands::search(&query)?,
    }

    Ok(())
}
