use super::Command;
use crate::{
    config::Config,
    filecache::{FileCache, NAME_FILECACHE},
    success,
};
use anyhow::Result;
use clap::Args;
use log::{debug, error};
use std::{fs, path::Path};

/// Removes applied dotfiles from the home directory
#[derive(Args)]
pub struct Clean {
    /// Force remove even if consistency errors occur
    #[arg(short, long)]
    force: bool,
}

impl Command for Clean {
    fn run(&self, cfg: &Config) -> Result<()> {
        let mut fc = FileCache::open(cfg.cache_dir.as_ref().join(NAME_FILECACHE))?;

        let mut failed = vec![];

        for f in fc.get() {
            debug!("delete {f:?}");
            if let Err(err) = fs::remove_file(f) {
                failed.push((f, err));
            }

            if let Some(parent) = f.parent() {
                if is_empty(parent)? {
                    debug!("delete {parent:?}/");
                    if let Err(err) = fs::remove_dir(parent) {
                        error!("failed deleting empty directory {parent:?}: {err}")
                    }
                }
            }
        }

        if !self.force && !failed.is_empty() {
            error!("Some cleanup operations failed:");
            for (p, err) in &failed {
                error!("  - {}: {}", p.to_string_lossy(), err);
            }
        }

        match self.force {
            true => fc.clear(),
            false => fc.set(failed.into_iter().map(|(p, _)| p.clone()).collect()),
        }

        fc.store()?;

        success!("Dotfiles have been removed from home directory.");

        Ok(())
    }
}

fn is_empty(p: &Path) -> Result<bool> {
    Ok(p.read_dir()?.next().is_none())
}
