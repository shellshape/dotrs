use super::Command;
use crate::{config::Config, service::Service as WatchService};
use anyhow::Result;
use clap::Args;
use duration_string::DurationString;
use log::LevelFilter;

#[derive(Args)]
pub struct Service {
    #[arg(long, default_value = "1s")]
    apply_delay: DurationString,

    #[arg(long, default_value = "5m")]
    update_delay: DurationString,

    #[arg(long, default_value = "10m")]
    pull_frequency: DurationString,
}

impl Command for Service {
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
