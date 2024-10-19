use super::Command;
use crate::{config::Config, success, util::dotfiles};
use anyhow::Result;
use clap::Args;
use log::warn;

/// Commit and push changes in stage to upstream repository
#[derive(Args)]
pub struct Update {
    /// Commit message
    #[arg(short, long)]
    message: Option<String>,

    /// Commit author
    #[arg(short, long, default_value = "dotrs <dot@rs>")]
    author: String,
}

impl Command for Update {
    fn run(&self, cfg: &Config) -> Result<()> {
        match dotfiles::update(cfg, &self.author, self.message.as_ref())? {
            true => {
                success!("Dotfiles stage changes have been published to remote repository.")
            }
            false => warn!("No changes have been made to stage dotfiles."),
        }

        Ok(())
    }
}
