use super::Command;
use crate::{config::Config, success, util::dotfiles};
use anyhow::Result;
use clap::Args;
use log::warn;

/// Updates the dotfiles stage from the upstream repository
#[derive(Args)]
pub struct Pull;

impl Command for Pull {
    fn run(&self, cfg: &Config) -> Result<()> {
        dotfiles::pull(cfg)?;
        success!("Dotfiles stage has been updated from upstream.");
        Ok(())
    }
}
