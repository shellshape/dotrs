use super::git::{self, Git};
use crate::{
    config::Config,
    filecache::{FileCache, NAME_FILECACHE},
};
use anyhow::Result;
use log::debug;
use std::fs;

pub fn apply(cfg: &Config) -> Result<()> {
    let home_dir =
        dirs::home_dir().ok_or_else(|| anyhow::anyhow!("failed getting user home dir"))?;

    debug!("home dir = {home_dir:?}");

    let copied_files = super::fs::copy_recursively(&cfg.stage_dir, &home_dir)?;

    let mut fc = FileCache::open(cfg.cache_dir.as_ref().join(NAME_FILECACHE))?;

    let diff = fc.diff(&copied_files);
    for f in diff.removed {
        debug!("delete {f:?}");
        fs::remove_file(f)?;
    }

    fc.set(copied_files);

    fc.store()?;

    Ok(())
}

pub fn pull(cfg: &Config) -> Result<()> {
    let cache_dir = cfg.stage_dir.as_ref();

    let git = Git::new(cache_dir);

    let branch = git.current_branch()?;

    git.exec(["pull", "origin", &branch])?;

    Ok(())
}

pub fn update(cfg: &Config) -> Result<bool> {
    let cache_dir = cfg.stage_dir.as_ref();

    let git = Git::new(cache_dir);

    let branch = git.current_branch()?;

    git.exec(["pull", "origin", &branch])?;

    match git.exec(["diff", "--quiet", "--exit-code"]) {
        Ok(_) => return Ok(false),
        Err(git::Error::NonZeroExit {
            code: 1,
            message: _,
        }) => { /* noop */ }
        Err(err) => return Err(err.into()),
    }

    git.exec(["add", "."])?;
    git.exec([
        "commit",
        "--message",
        "auto-update dotfiles",
        "--author",
        "dotrs <noreply@dot.rs>",
    ])?;
    git.exec(["push", "origin", &branch])?;

    Ok(true)
}
