#![feature(cursor_remaining)]

mod args;
mod chunk;
mod chunk_type;
mod commands;
mod png;
use clap::{Parser, Subcommand};

/// Hide messages in PNG files
#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
struct Cli {
    #[clap(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Encode a PNG file
    Encode {
        #[clap(value_parser)]
        file_name: String,
        #[clap(value_parser)]
        chunk_type: String,
        #[clap(value_parser)]
        payload: String,
    },
    /// Decodes a PNG file
    Decode {
        #[clap(value_parser)]
        file_name: String,
        #[clap(value_parser)]
        chunk_type: String,
    },
    /// Removes a chunk given a chunk type
    Remove {
        #[clap(value_parser)]
        file_name: String,
        #[clap(value_parser)]
        chunk_type: String,
    },

}

pub type Error = Box<dyn std::error::Error>;
pub type Result<T> = std::result::Result<T, Error>;

fn main() -> Result<()> {
    let cli = Cli::parse();


    match &cli.command {
        Commands::Encode {
            file_name,
            chunk_type,
            payload,
        } => {
            if args::encode(file_name, chunk_type, payload).is_err() {
                std::fs::remove_file(&format!("{}.temp", file_name))?;
            }
        }
        Commands::Decode {
            file_name,
            chunk_type,
        } => {
            args::decode(file_name, chunk_type)?;
        }

        Commands::Remove {
            file_name,
            chunk_type,
        } => {
            if args::remove(file_name, chunk_type).is_err() {
                std::fs::remove_file(&format!("{}.temp", file_name))?;
            }
        }
    }

    Ok(())
}
