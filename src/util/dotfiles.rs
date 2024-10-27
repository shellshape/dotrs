use super::git::{self, Change, Git};
use crate::config::Config;
use crate::filecache::{FileCache, NAME_FILECACHE};
use crate::profile::{get_applied_profile, write_applied_profile, Profile, Value};
use anyhow::Result;
use handlebars::Handlebars;
use ignore::{DirEntry, WalkBuilder};
use log::debug;
use std::fs::{self, File};
use std::io::{Read, Write};
use std::path::{Path, PathBuf};

pub fn apply<S: Into<String>>(cfg: &Config, profile: Option<S>) -> Result<()> {
    let home_dir = super::home_dir()?;

    debug!("home_dir = {home_dir:?}");

    let profile = match profile {
        Some(p) => Some(p.into()),
        None => get_applied_profile(&cfg.cache_dir)?,
    };

    debug!("profile = {profile:?}");

    let copied_files = apply_recursively(&cfg.stage_dir, &home_dir, profile.as_deref())?;

    let mut fc = FileCache::open(cfg.cache_dir.as_ref().join(NAME_FILECACHE))?;

    let diff = fc.diff(&copied_files);
    for f in diff.removed {
        debug!("delete {f:?}");
        fs::remove_file(f)?;
    }

    fc.set(copied_files);

    fc.store()?;

    if let Some(profile) = profile {
        debug!("writing profile {profile} to cache ...");
        write_applied_profile(&cfg.cache_dir, &profile)?;
    }

    Ok(())
}

pub fn pull(cfg: &Config) -> Result<()> {
    let cache_dir = cfg.stage_dir.as_ref();

    let git = Git::new(cache_dir);

    let branch = git.current_branch()?;

    git.exec(["pull", "origin", &branch])?;

    Ok(())
}

pub fn update(
    cfg: &Config,
    author: impl AsRef<str>,
    message: Option<impl AsRef<str>>,
) -> Result<bool> {
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

    let message = match message {
        Some(ref s) => s.as_ref(),
        None => {
            let changed_files = git
                .changed_files()?
                .iter()
                .map(change_to_string)
                .collect::<Vec<_>>()
                .join(", ");
            &format!("auto-update: {changed_files}")
        }
    };

    git.exec(["commit", "--message", message, "--author", author.as_ref()])?;
    git.exec(["push", "origin", &branch])?;

    Ok(true)
}

fn change_to_string((mode, filename): &(Change, String)) -> String {
    let prefix = match mode {
        Change::Modified => "update",
        Change::Added => "add",
        Change::Deleted => "remove",
    };
    format!("{prefix} {filename}")
}

pub fn apply_recursively(
    from: impl AsRef<Path>,
    to: impl AsRef<Path>,
    profile: Option<&str>,
) -> Result<Vec<PathBuf>> {
    let walker = WalkBuilder::new(&from)
        .hidden(false)
        .add_custom_ignore_filename(".dotrsignore")
        .filter_entry(walk_filter)
        .build();

    let mut copied_files = vec![];

    let mut hb = Handlebars::new();
    hb.set_strict_mode(true);

    let data = match profile {
        Some(profile) => Profile::new(from.as_ref(), profile != "default").load(profile)?,
        None => Value::None,
    };

    let mut buf = String::new();

    for entry in walker {
        let entry = entry?;
        let path = entry.path();
        let to_path = to.as_ref().join(path.strip_prefix(&from)?);

        if path.metadata()?.is_dir() {
            if !to_path.exists() {
                fs::create_dir_all(&to_path)?;
            }
        } else {
            buf.clear();
            File::open(path)?.read_to_string(&mut buf)?;
            let rendered = hb.render_template(&buf, &data)?;
            File::create(&to_path)?.write_all(rendered.as_bytes())?;

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
        true => !de.path().ends_with(".git") && !de.path().ends_with(".dotrs-profiles"),
        false => !de.path().ends_with(".gitignore"),
    }
}
