use super::Command;
use crate::config::Config;
use crate::filecache::{FileCache, NAME_FILECACHE};
use anyhow::Result;
use clap::Args;
use log::warn;

/// List currently applied dotfiles
#[derive(Args)]
#[command(aliases = ["ls"])]
pub struct List;

impl Command for List {
    fn run(&self, cfg: &Config) -> Result<()> {
        let fc = FileCache::open(cfg.cache_dir.as_ref().join(NAME_FILECACHE))?;
        let entries = fc.get();

        if entries.is_empty() {
            warn!("No applied dotfiles.");
            return Ok(());
        }

        for entry in entries {
            println!("{}", entry.to_string_lossy());
        }

        Ok(())
    }
}
