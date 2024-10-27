pub mod dotfiles;
pub mod git;

use anyhow::Result;
use std::path::PathBuf;

pub fn home_dir() -> Result<PathBuf> {
    dirs::home_dir().ok_or_else(|| anyhow::anyhow!("failed getting home directory"))
}
