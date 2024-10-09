use anyhow::Result;
use std::{
    fs::{self, File},
    io::{read_to_string, Write},
    path::{Path, PathBuf},
    str::FromStr,
};

pub const NAME_FILECACHE: &str = "tracked_files";

pub struct FileCache {
    files: Vec<PathBuf>,
    storage_dir: PathBuf,
}

#[allow(unused)]
pub struct Diff<T> {
    pub added: Vec<T>,
    pub removed: Vec<T>,
}

impl FileCache {
    pub fn open<P: Into<PathBuf>>(path: P) -> Result<Self> {
        let storage_dir = path.into();
        let files = match storage_dir.exists() {
            true => parse(&storage_dir)?,
            false => vec![],
        };
        Ok(Self { storage_dir, files })
    }

    pub fn store(&self) -> Result<()> {
        if let Some(parent) = self.storage_dir.parent() {
            if !parent.exists() {
                fs::create_dir_all(parent)?;
            }
        }

        let mut f = File::create(&self.storage_dir)?;
        for entry in &self.files {
            writeln!(f, "{}", entry.to_string_lossy())?;
        }

        Ok(())
    }

    pub fn get(&self) -> &Vec<PathBuf> {
        &self.files
    }

    pub fn set(&mut self, v: Vec<PathBuf>) {
        self.files = v;
    }

    pub fn clear(&mut self) {
        self.set(vec![]);
    }

    pub fn diff<'a, 'b: 'a>(&'a self, other: &'b [PathBuf]) -> Diff<&'a PathBuf> {
        let added = other.iter().filter(|o| !self.files.contains(o)).collect();
        let removed = self.files.iter().filter(|s| !other.contains(s)).collect();
        Diff { added, removed }
    }
}

fn parse(p: &Path) -> Result<Vec<PathBuf>> {
    let f = File::open(p)?;
    let entries: Result<Vec<_>, _> = read_to_string(f)?
        .lines()
        .map(|s| s.trim())
        .filter(|s| !s.is_empty())
        .map(PathBuf::from_str)
        .collect();
    Ok(entries?)
}
