use super::Command;
use crate::config::Config;
use anyhow::Result;
use clap::Args;

const LONG_ABOUT: &str = "\
    This command does only print the configured path to the stage directory.\n\
    You can use this command with a wrapper script for your shell to use the\n\
    output directory to change your shell directory to the output directory.\n\
    \n\
    Following, you can find an example script for bash-like shells. Feel free\n\
    to use or adapt the script as you desire.\n\
    https://github.com/shellshape/dotrs/blob/main/bin/wrapper-bash.sh";

/// Utility to change directory to the stage directory
#[derive(Args)]
#[command(long_about = LONG_ABOUT)]
pub struct Cd;

impl Command for Cd {
    fn run(&self, cfg: &Config) -> Result<()> {
        println!("{}", cfg.stage_dir.as_ref().to_string_lossy());
        Ok(())
    }
}
