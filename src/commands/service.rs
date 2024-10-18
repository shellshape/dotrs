use super::Command;
use crate::{config::Config, service::Service as WatchService};
use anyhow::Result;
use clap::Args;
use duration_string::DurationString;
use log::LevelFilter;

/// Start up the synchronization service
#[derive(Args)]
#[command(visible_aliases = ["service"], )]
pub struct StartService {
    /// The debounce-delay for applying dotfiles from stage to home dir
    #[arg(long, default_value = "1s")]
    apply_delay: DurationString,

    /// The debounce-delay for updating the upstream repository from stage
    #[arg(long, default_value = "5m")]
    update_delay: DurationString,

    /// The frequency to pull changes from the upstream repo to stage
    #[arg(long, default_value = "10m")]
    pull_frequency: DurationString,
}

impl Command for StartService {
    fn run(&self, _: &Config) -> Result<()> {
        let s = WatchService::new(
            self.apply_delay.into(),
            self.update_delay.into(),
            self.pull_frequency.into(),
        )?;
        s.watch()?;
        Ok(())
    }

    fn init_logger(&self, level_filter: LevelFilter) {
        env_logger::Builder::new()
            .filter_level(level_filter)
            .format_module_path(false)
            .format_target(false)
            .init();
    }
}
