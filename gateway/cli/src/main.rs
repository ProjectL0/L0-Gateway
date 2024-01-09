//! L0-Gateway Command Line Interface

use clap::Parser;
use cli::CliArgs;

fn main() {
    let cli = CliArgs::parse();
    println!("Hello, world!");
}
