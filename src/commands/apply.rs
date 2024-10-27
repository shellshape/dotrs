use super::Command;
use crate::{config::Config, success, util::dotfiles};
use anyhow::Result;
use clap::Args;

const LONG_ABOUT: &str = "Apply dotfiles from stage to home directory and apply templates \
    according to the selected profile. If no profile is selected, the already applied profile \
    will be re-applied. If no profile has been applied before and no profile is selected, no \
    profile will be applied to the dotfiles. \n\
    \n\
    When variables are missing in a profile for the templates in stage, the operation will fail.";

/// Apply dotfiles from stage to home directory and apply templates
#[derive(Args)]
#[command(visible_aliases = ["a"], long_about = LONG_ABOUT)]
pub struct Apply {
    // The profile to be applied
    #[arg(short, long)]
    profile: Option<String>,
}

impl Command for Apply {
    fn run(&self, cfg: &Config) -> Result<()> {
        dotfiles::apply(cfg, self.profile.as_ref())?;
        success!("Dotfiles applied from stage.");
        Ok(())
    }
}
