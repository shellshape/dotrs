use super::Command;
use crate::config::Config;
use anyhow::Result;
use clap::Args;

#[derive(Args)]
pub struct Cd {}

impl Command for Cd {
    fn run(&self, cfg: &Config) -> Result<()> {
        println!("{}", cfg.stage_dir.as_ref().to_string_lossy());
        Ok(())
    }
}
