use super::Command;
use crate::util::dotfiles;
use crate::{config::Config, success, util::git::Git};
use anyhow::{Ok, Result};
use clap::Args;
use std::fs;

/// Import dotfiles from an external git repository
#[derive(Args)]
#[command(visible_aliases = ["i"])]
pub struct Import {
    /// The URI of the source Git repository
    uri: String,

    /// The branch to check out
    #[arg(short, long, default_value = "main")]
    branch: String,
}

impl Command for Import {
    fn run(&self, cfg: &Config) -> Result<()> {
        import_from_git(cfg, &self.uri, &self.branch)?;
        Ok(())
    }
}

fn import_from_git(cfg: &Config, url: &str, git_ref: &str) -> Result<()> {
    if !cfg.stage_dir.as_ref().exists() {
        fs::create_dir_all(&cfg.stage_dir)?;
    } else if dotfiles::stage_dir_initialized(cfg) {
        anyhow::bail!("Stage dir already contains a git repository")
    }

    let git = Git::new(cfg.stage_dir.as_ref());
    git.exec(["init"])?;
    git.exec(["remote", "add", "origin", url])?;
    git.exec(["fetch", "--all"])?;
    git.exec(["checkout", git_ref])?;

    success!("Dotfiles have been imported to stage directory.");

    Ok(())
}
