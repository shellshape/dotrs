use env_logger::fmt::Formatter;
use log::{Level, Record};
use std::io::{self, Write};
use yansi::Paint;

#[macro_export]
macro_rules! success {
    ($msg:expr) => {
        log::info!("{}", yansi::Paint::green($msg))
    };
}

pub fn init_cli_logger(level_filter: log::LevelFilter) {
    env_logger::Builder::new()
        .filter_level(level_filter)
        .format(cli_formatter)
        .init();
}

fn cli_formatter(buf: &mut Formatter, rec: &Record<'_>) -> io::Result<()> {
    match rec.level() {
        Level::Error => writeln!(buf, "{} {}", "error:".red().bold(), rec.args()),
        Level::Warn => writeln!(buf, "{} {}", "warning:".yellow().bold(), rec.args()),
        Level::Info => writeln!(buf, "{}", rec.args()),
        _ => writeln!(buf, "{}", rec.args().dim().italic()),
    }
}
