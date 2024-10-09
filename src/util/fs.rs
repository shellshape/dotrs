use anyhow::Result;
use ignore::{DirEntry, WalkBuilder};
use log::debug;
use std::{
    fs,
    path::{Path, PathBuf},
};

pub fn copy_recursively(from: impl AsRef<Path>, to: impl AsRef<Path>) -> Result<Vec<PathBuf>> {
    let walker = WalkBuilder::new(&from)
        .hidden(false)
        .add_custom_ignore_filename(".dotrsignore")
        .filter_entry(walk_filter)
        .build();

    let mut copied_files = vec![];

    for entry in walker {
        let entry = entry?;
        let path = entry.path();
        let to_path = to.as_ref().join(path.strip_prefix(&from)?);

        if path.metadata()?.is_dir() {
            if !to_path.exists() {
                fs::create_dir_all(&to_path)?;
            }
        } else {
            fs::copy(path, &to_path)?;
            copied_files.push(to_path.to_owned());
            debug!("copied {path:?} -> {to_path:?}");
        }
    }

    Ok(copied_files)
}

fn walk_filter(de: &DirEntry) -> bool {
    let Ok(meta) = de.metadata() else {
        return false;
    };

    match meta.is_dir() {
        true => !de.path().ends_with(".git"),
        false => !de.path().ends_with(".gitignore"),
    }
}
