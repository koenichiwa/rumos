mod args;
mod funcs;

use futures::executor;
use clap::Parser;

use crate::args::{Cli, ClapCommands};
use crate::funcs::{Command, BrightnessCommand};


const MAX_BRIGHTNESS: u32 = 100;
const MIN_BRIGHTNESS: u32 = 5;

impl From<ClapCommands> for Command {
    fn from(value: ClapCommands) -> Self {
        match value {
            ClapCommands::Get { devices } => Command::BrightnessCommand { command: BrightnessCommand::Get, devices },
            ClapCommands::Set { percent, devices } => Command::BrightnessCommand { command: BrightnessCommand::Set { percent }, devices },
            ClapCommands::Inc { percent, devices } => Command::BrightnessCommand { command: BrightnessCommand::Inc { percent }, devices },
            ClapCommands::Dec { percent, devices } => Command::BrightnessCommand { command: BrightnessCommand::Dec { percent }, devices },
            ClapCommands::Max { devices } => Command::BrightnessCommand { command: BrightnessCommand::Max, devices },
            ClapCommands::Min { devices } => Command::BrightnessCommand { command: BrightnessCommand::Min, devices },
        }
    }
}

fn main() -> Result<(), brightness::Error> {
    let cli = Cli::parse();
    executor::block_on(Command::from(cli.command).handle(cli.quiet, cli.percent))
}