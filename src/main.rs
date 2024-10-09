mod commands;
mod config;
mod filecache;
mod logging;
mod service;
mod util;

use anyhow::Result;
use clap::{command, Parser};
use commands::*;
use config::Config;
use log::LevelFilter;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[arg(short, long)]
    verbose: bool,

    #[command(subcommand)]
    commands: Commands,
}

register_commands! {
    Apply
    Import
    Clean
    Cd
    Service
    Update
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    let cfg = Config::parse()?;

    let level_filter = match cli.verbose {
        true => LevelFilter::Debug,
        false => cfg.log_level,
    };

    cli.commands.init_logger(level_filter);
    cli.commands.run(&cfg)?;

    Ok(())
}
