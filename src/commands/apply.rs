use super::Command;
use crate::{config::Config, success, util::dotfiles};
use anyhow::Result;
use clap::Args;
use log::info;

#[derive(Args)]
#[command(visible_aliases = ["a"])]
pub struct Apply;

impl Command for Apply {
    fn run(&self, cfg: &Config) -> Result<()> {
        dotfiles::apply(cfg)?;
        success!("Dotfiles applied from stage.");
        Ok(())
    }
}
