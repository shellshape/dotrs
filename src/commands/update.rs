use super::Command;
use crate::{config::Config, success, util::dotfiles};
use anyhow::Result;
use clap::Args;
use log::warn;

#[derive(Args)]
pub struct Update {}

impl Command for Update {
    fn run(&self, cfg: &Config) -> Result<()> {
        match dotfiles::update(cfg)? {
            true => {
                success!("Dotfiles stage changes have been published to remote repository.")
            }
            false => warn!("No changes have been made to stage dotfiles."),
        }

        Ok(())
    }
}
