use std::path::PathBuf;

use clap::{Parser, Subcommand};


#[derive(Parser, Debug)]
#[command(author, version, about)]
pub struct CliArgs {

    /// Set a custom config file.
    #[arg(short, long, value_name = "FILE_PATH")]
    config: Option<PathBuf>,
    
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand, Debug)]
pub enum Commands {}
