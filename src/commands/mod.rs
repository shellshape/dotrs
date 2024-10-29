use crate::{config::Config, logging};
use anyhow::Result;
use log::LevelFilter;

macro_rules! re_export {
    ( $( $md:tt )+ ) => {
        $(
            mod $md;
            pub use $md::*;
        )*
    };
}

// List the names of your command modules to re-export them
// in this module.
re_export! {
    apply
    cd
    clean
    import
    list
    pull
    service
    update
}

pub trait Command {
    fn run(&self, cfg: &Config) -> Result<()>;

    fn init_logger(&self, level_filter: LevelFilter) {
        logging::init_cli_logger(level_filter);
    }
}

#[macro_export]
macro_rules! register_commands {
    ( $( $command:tt )+ ) => {
        #[derive(clap::Subcommand)]
        enum Commands {
            $(
                $command($command),
            )*
        }

        impl std::ops::Deref for Commands {
            type Target = dyn $crate::commands::Command;

            fn deref(&self) -> &Self::Target {
                match &self {
                    $(
                        Self::$command(c) => c,
                    )*
                }
            }
        }
    };
}
