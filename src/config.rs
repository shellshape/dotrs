use anyhow::Result;
use envconfig::Envconfig;
use std::{
    path::{Path, PathBuf},
    str::FromStr,
};

macro_rules! path_with_home_default {
    ($name:tt, $def:literal) => {
        pub struct $name(PathBuf);

        impl FromStr for $name {
            type Err = anyhow::Error;

            fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
                if s.is_empty() {
                    Ok(Self(
                        dirs::home_dir()
                            .ok_or_else(|| anyhow::anyhow!("could not find user home path"))?
                            .join($def),
                    ))
                } else {
                    Ok(Self(PathBuf::from_str(s)?))
                }
            }
        }

        impl AsRef<Path> for $name {
            fn as_ref(&self) -> &Path {
                self.0.as_ref()
            }
        }
    };
}

path_with_home_default! { StageDir, ".local/dotrs/stage" }
path_with_home_default! { CacheDir, ".local/dotrs/cache" }

#[derive(Envconfig)]
pub struct Config {
    #[envconfig(from = "DOTRS_STAGE_DIR", default = "")]
    pub stage_dir: StageDir,

    #[envconfig(from = "DOTRS_CACHE_DIR", default = "")]
    pub cache_dir: CacheDir,

    #[envconfig(from = "DOTRS_LOG_LEVEL", default = "info")]
    pub log_level: log::LevelFilter,
}

impl Config {
    pub fn parse() -> Result<Self> {
        dotenv::dotenv().ok();
        Ok(Self::init_from_env()?)
    }
}
