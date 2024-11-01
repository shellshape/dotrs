use super::Command;
use crate::config::Config;
use anyhow::Result;
use clap::Args;

#[derive(Args)]
pub struct Encrypt {
    value: String,

    #[arg(short, long)]
    key: Option<String>,
}

impl Command for Encrypt {
    fn run(&self, cfg: &Config) -> Result<()> {
        println!("{}", cfg.stage_dir.as_ref().to_string_lossy());
        Ok(())
    }
}
