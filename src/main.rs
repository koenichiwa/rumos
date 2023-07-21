mod args;
mod funcs;

use std::collections::HashSet;
use std::sync::Arc;

use clap::Parser;
use futures::executor;

use crate::args::{Cli, Command as CliCommand};
use crate::funcs::{BrightnessCommand, Command as FuncsCommand, DeviceSelector};

const MAX_BRIGHTNESS: u32 = 100;
const MIN_BRIGHTNESS: u32 = 5;
const MAX_CONCURRENCY: Option<usize> = Some(5);

fn get_selector(names: Option<Vec<String>>) -> DeviceSelector {
    if let Some(names) = names {
        return DeviceSelector::ByName(Arc::<HashSet<String>>::new(names.into_iter().collect()));
    }
    DeviceSelector::All
}

impl From<CliCommand> for FuncsCommand {
    fn from(value: CliCommand) -> Self {
        match value {
            CliCommand::Get { devices } => FuncsCommand::BrightnessCommand {
                command: BrightnessCommand::Get,
                selector: get_selector(devices),
            },
            CliCommand::Set { percent, devices } => FuncsCommand::BrightnessCommand {
                command: BrightnessCommand::Set { percent },
                selector: get_selector(devices),
            },
            CliCommand::Inc { percent, devices } => FuncsCommand::BrightnessCommand {
                command: BrightnessCommand::Inc { percent },
                selector: get_selector(devices),
            },
            CliCommand::Dec { percent, devices } => FuncsCommand::BrightnessCommand {
                command: BrightnessCommand::Dec { percent },
                selector: get_selector(devices),
            },
            CliCommand::Max { devices } => FuncsCommand::BrightnessCommand {
                command: BrightnessCommand::Max,
                selector: get_selector(devices),
            },
            CliCommand::Min { devices } => FuncsCommand::BrightnessCommand {
                command: BrightnessCommand::Min,
                selector: get_selector(devices),
            },
            CliCommand::List => FuncsCommand::List,
        }
    }
}

fn main() -> Result<(), brightness::Error> {
    let cli = Cli::parse();
    executor::block_on(FuncsCommand::from(cli.command).handle(cli.quiet, cli.percent))
}
