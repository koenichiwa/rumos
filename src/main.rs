mod args;
mod funcs;

use std::collections::HashSet;
use std::sync::Arc;

use clap::Parser;
use funcs::DeviceSelector;
use futures::executor;

use crate::args::{ClapCommands, Cli};
use crate::funcs::{BrightnessCommand, Command};

const MAX_BRIGHTNESS: u32 = 100;
const MIN_BRIGHTNESS: u32 = 5;
const MAX_CONCURRENCY: Option<usize> = Some(5);

fn get_selector(names: Option<Vec<String>>) -> DeviceSelector {
    if let Some(names) = names {
        return DeviceSelector::ByName(Arc::<HashSet<String>>::new(names.into_iter().collect()));
    }
    DeviceSelector::All
}

impl From<ClapCommands> for Command {
    fn from(value: ClapCommands) -> Self {
        match value {
            ClapCommands::Get { devices } => Command::BrightnessCommand {
                command: BrightnessCommand::Get,
                selector: get_selector(devices),
            },
            ClapCommands::Set { percent, devices } => Command::BrightnessCommand {
                command: BrightnessCommand::Set { percent },
                selector: get_selector(devices),
            },
            ClapCommands::Inc { percent, devices } => Command::BrightnessCommand {
                command: BrightnessCommand::Inc { percent },
                selector: get_selector(devices),
            },
            ClapCommands::Dec { percent, devices } => Command::BrightnessCommand {
                command: BrightnessCommand::Dec { percent },
                selector: get_selector(devices),
            },
            ClapCommands::Max { devices } => Command::BrightnessCommand {
                command: BrightnessCommand::Max,
                selector: get_selector(devices),
            },
            ClapCommands::Min { devices } => Command::BrightnessCommand {
                command: BrightnessCommand::Min,
                selector: get_selector(devices),
            },
            ClapCommands::List => Command::List,
        }
    }
}

fn main() -> Result<(), brightness::Error> {
    let cli = Cli::parse();
    executor::block_on(Command::from(cli.command).handle(cli.quiet, cli.percent))
}
