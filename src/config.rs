use clap::Args;
use log::LevelFilter;
use std::fmt;
use std::fmt::Formatter;
use std::path::{Path, PathBuf};
use std::str::FromStr;

macro_rules! default_home_dir {
    ( $first_elem:literal $( / $elem:literal )* ) => {{
        PrintablePathBuf(
            dirs::home_dir().expect("home dir")
            .join($first_elem)
            $( .join($elem) )*
        )
    }};
}

#[derive(Clone)]
pub struct PrintablePathBuf(PathBuf);

impl FromStr for PrintablePathBuf {
    type Err = <PathBuf as FromStr>::Err;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(PathBuf::from_str(s)?))
    }
}

impl fmt::Display for PrintablePathBuf {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        self.0.to_string_lossy().fmt(f)
    }
}

impl AsRef<Path> for PrintablePathBuf {
    fn as_ref(&self) -> &Path {
        self.0.as_ref()
    }
}

#[derive(Args, Clone)]
pub struct Config {
    #[arg(short, long, default_value = "info", env = "DOTRS_LOG_LEVEL")]
    pub log_level: LevelFilter,

    #[arg(
        long,
        default_value_t = default_home_dir!(".local" / "dotrs" / "stage"),
        env = "DOTRS_STAGE_DIR"
    )]
    pub stage_dir: PrintablePathBuf,

    #[arg(
        long,
        default_value_t = default_home_dir!(".local" / "dotrs" / "cache"),
        env = "DOTRS_CACHE_DIR"
    )]
    pub cache_dir: PrintablePathBuf,
}
