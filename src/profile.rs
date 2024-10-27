use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::{
    collections::HashMap,
    fs::{self, File},
    io::{self, ErrorKind, Read, Write},
    path::{Path, PathBuf},
};

const PROFILE_DIR: &str = ".dotrs-profiles";
const APPLIED_PROFILE_FILE: &str = ".dotrs-applied-profile";

#[allow(dead_code)]
#[derive(Serialize, Deserialize)]
#[serde(untagged)]
pub enum Value {
    String(String),
    Int(isize),
    Float(f64),
    Bool(bool),
    List(Vec<Value>),
    Map(HashMap<String, Value>),
    None,
}

impl Value {
    pub fn parse<R: io::Read>(rdr: R) -> Result<Self> {
        Ok(serde_yaml::from_reader(rdr)?)
    }
}

pub struct Profile {
    base_path: PathBuf,
    must_exist: bool,
}

impl Profile {
    pub fn new<P: Into<PathBuf>>(base_path: P, must_exist: bool) -> Self {
        Self {
            base_path: base_path.into(),
            must_exist,
        }
    }

    pub fn load(&self, name: &str) -> Result<Value> {
        let f = self
            .open_profile_file(&format!("{name}.yaml"))
            .transpose()
            .or_else(|| self.open_profile_file(&format!("{name}.yml")).transpose())
            .transpose()?;

        match f {
            Some(f) => Value::parse(f),
            None if self.must_exist => Err(anyhow::anyhow!("No profile exists with name '{name}'")),
            _ => Ok(Value::None),
        }
    }

    fn open_profile_file(&self, name: &str) -> Result<Option<File>> {
        let res = File::open(self.base_path.join(PROFILE_DIR).join(name));
        match res {
            Ok(f) => Ok(Some(f)),
            Err(err) if err.kind() == ErrorKind::NotFound => Ok(None),
            Err(err) => Err(err.into()),
        }
    }
}

pub fn get_applied_profile<P: AsRef<Path>>(cache_dir: P) -> Result<Option<String>> {
    let p = cache_dir.as_ref().join(APPLIED_PROFILE_FILE);
    match read_file_to_string(p) {
        Ok(v) => Ok(Some(v)),
        Err(err) if err.kind() == ErrorKind::NotFound => Ok(None),
        Err(err) => Err(err.into()),
    }
}

pub fn write_applied_profile<P: AsRef<Path>>(cache_dir: P, name: &str) -> Result<()> {
    let cache_dir = cache_dir.as_ref();
    if !cache_dir.exists() {
        fs::create_dir_all(cache_dir)?;
    }
    File::create(cache_dir.join(APPLIED_PROFILE_FILE))?.write_all(name.as_bytes())?;
    Ok(())
}

fn read_file_to_string<P: AsRef<Path>>(path: P) -> io::Result<String> {
    let mut f = File::open(path)?;
    let mut buf = String::new();
    f.read_to_string(&mut buf)?;
    Ok(buf.trim().to_string())
}
