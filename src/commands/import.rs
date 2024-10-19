use super::Command;
use crate::{config::Config, success, util::git::Git};
use anyhow::{Ok, Result};
use clap::Args;
use std::{fs, path::Path};

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
        import_from_git(&cfg.stage_dir, &self.uri, &self.branch)?;
        Ok(())
    }
}

fn import_from_git(stage_dir: impl AsRef<Path>, url: &str, git_ref: &str) -> Result<()> {
    if !stage_dir.as_ref().exists() {
        fs::create_dir_all(&stage_dir)?;
    } else if stage_dir.as_ref().join(".git").exists() {
        anyhow::bail!("Stage dir already contains a git repository")
    }

    let git = Git::new(stage_dir.as_ref());
    git.exec(["init"])?;
    git.exec(["remote", "add", "origin", url])?;
    git.exec(["fetch", "--all"])?;
    git.exec(["checkout", git_ref])?;

    success!("Dotfiles have been imported to stage directory.");

    Ok(())
}
