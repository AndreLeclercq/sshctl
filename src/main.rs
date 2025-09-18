mod config;
mod commands;

use anyhow::Result;
use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    #[command(about = "Manage connections with 'add', 'remove', 'edit' and 'list' subcommands")]
    Connection {
        #[command(subcommand)]
        connection_command: ConnectionCommands,
    }
}

#[derive(Subcommand)]
enum ConnectionCommands {
    #[command(about = "Add new connection to the config file")]
    Add {
        #[arg(required = true)]
        name: String,
    },
    #[command(about = "Remove connection to the config file")]
    Remove {
        #[arg(required = true)]
        name: String,
    },
    #[command(about = "Edit connection to the config file")]
    Edit {
        #[arg(required = true)]
        name: String,
    },
    #[command(about = "Show connection informations")]
    Show {
        #[arg(required = true)]
        name: String,
    }
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Connection { connection_command } => {
            match connection_command {
                ConnectionCommands::Add { name } => commands::connection::add(name)?,
                ConnectionCommands::Remove { name } => commands::connection::remove(name)?,
                ConnectionCommands::Edit { name } => commands::connection::edit(name)?,
                ConnectionCommands::Show { name } => commands::connection::show(name)?,
            }
        }
    }
    Ok(())
}
