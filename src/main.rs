mod commands;
mod config;
mod encryption;
mod filecache;
mod logging;
mod profile;
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

    #[clap(flatten)]
    config: Config,

    #[command(subcommand)]
    commands: Commands,
}

register_commands! {
    Apply
    Clean
    List
    Import
    Encrypt
    Pull
    Update
    Cd
    StartService
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    let cfg = cli.config;

    let level_filter = match cli.verbose {
        true => LevelFilter::Debug,
        false => cfg.log_level,
    };

    cli.commands.init_logger(level_filter);
    cli.commands.run(&cfg)?;

    Ok(())
}
