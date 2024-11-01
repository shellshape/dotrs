pub mod errors;

use crate::encryption;
use errors::{Error, Result};
use serde::{Deserialize, Serialize};
use std::{
    collections::HashMap,
    fs::{self, File},
    io::{self, ErrorKind, Read, Write},
    path::{Path, PathBuf},
};

pub const ENCRYPTED_VALUE_KEY: &str = "$encrypted";

const PROFILE_DIR: &str = ".dotrs-profiles";
const APPLIED_PROFILE_FILE: &str = ".dotrs-applied-profile";

#[derive(Deserialize)]
pub struct EncryptedValue {
    #[serde(rename = "$encrypted")]
    value: String,
}

impl EncryptedValue {
    pub fn decrypt(&self, key: &str) -> Result<Value> {
        Ok(Value::String(encryption::decrypt_string(&self.value, key)?))
    }
}

#[derive(Deserialize)]
#[serde(untagged)]
pub enum DecodeValue {
    String(String),
    Int(isize),
    Float(f64),
    Bool(bool),
    List(Vec<DecodeValue>),
    Map(HashMap<String, DecodeValue>),
    EncryptedValue(EncryptedValue),
    None,
}

impl DecodeValue {
    pub fn parse<R: io::Read>(rdr: R) -> Result<Self> {
        Ok(serde_yaml::from_reader(rdr)?)
    }

    pub fn contains_encrypted_value(&self) -> bool {
        match self {
            DecodeValue::List(vec) => vec.iter().any(|v| v.contains_encrypted_value()),
            DecodeValue::Map(hash_map) => {
                hash_map.iter().any(|(_, v)| v.contains_encrypted_value())
            }
            DecodeValue::EncryptedValue(_) => true,
            _ => false,
        }
    }

    pub fn decrypt(self, key: Option<&str>) -> Result<Value> {
        Ok(match self {
            DecodeValue::None => Value::None,
            DecodeValue::String(v) => Value::String(v),
            DecodeValue::Int(v) => Value::Int(v),
            DecodeValue::Float(v) => Value::Float(v),
            DecodeValue::Bool(v) => Value::Bool(v),
            DecodeValue::List(vec) => {
                let list: Result<Vec<_>> = vec.into_iter().map(|v| v.decrypt(key)).collect();
                Value::List(list?)
            }
            DecodeValue::Map(hash_map) => {
                let kv: Result<HashMap<_, _>> = hash_map
                    .into_iter()
                    .map(|(k, v)| v.decrypt(key).map(|v| (k, v)))
                    .collect();
                Value::Map(kv?)
            }
            DecodeValue::EncryptedValue(encrypted_value) => {
                let key = key.ok_or(Error::NoEncryptionKey)?;
                encrypted_value.decrypt(key)?
            }
        })
    }
}

#[allow(dead_code)]
#[derive(Serialize)]
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

    pub fn load(&self, name: &str) -> Result<DecodeValue> {
        let f = self
            .open_profile_file(&format!("{name}.yaml"))
            .transpose()
            .or_else(|| self.open_profile_file(&format!("{name}.yml")).transpose())
            .transpose()?;

        match f {
            Some(f) => DecodeValue::parse(f),
            None if self.must_exist => Err(Error::NoProfileWithName(name.to_string())),
            _ => Ok(DecodeValue::None),
        }
    }

    fn open_profile_file(&self, name: &str) -> Result<Option<File>> {
        let path = self.base_path.join(PROFILE_DIR).join(name);
        let res = File::open(&path);
        match res {
            Ok(f) => Ok(Some(f)),
            Err(err) if err.kind() == ErrorKind::NotFound => Ok(None),
            Err(err) => Err(err.into()),
        }
    }
}

pub fn get_applied_profile<P: AsRef<Path>>(cache_dir: P) -> Result<Option<String>> {
    let path = cache_dir.as_ref().join(APPLIED_PROFILE_FILE);
    match read_file_to_string(&path) {
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
