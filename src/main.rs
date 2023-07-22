mod args;
mod error;
mod funcs;

use std::collections::HashSet;
use std::sync::Arc;

use clap::Parser;
use futures::executor;

use args::{
    BrightnessOutput as CliBrightnessOutput, Cli, Command as CliCommand,
    DeviceSelector as CliDeviceSelector,
};
pub use error::Error;
use funcs::{
    BrightnessCommand, BrightnessOutput as FuncsBrightnessOutput, Command as FuncsCommand,
    DeviceSelector as FuncsDeviceSelector,
};

const MAX_BRIGHTNESS: u32 = 100;
const MIN_BRIGHTNESS: u32 = 5;
const MAX_CONCURRENCY: Option<usize> = Some(5);

impl From<CliBrightnessOutput> for FuncsBrightnessOutput {
    fn from(value: CliBrightnessOutput) -> Self {
        match value {
            CliBrightnessOutput {
                quiet: false,
                percent: false,
            } => FuncsBrightnessOutput::Default,
            CliBrightnessOutput {
                quiet: true,
                percent: false,
            } => FuncsBrightnessOutput::Quiet,
            CliBrightnessOutput {
                quiet: false,
                percent: true,
            } => FuncsBrightnessOutput::Percent,
            CliBrightnessOutput { .. } => unreachable!("The variables are mutually exclusive"),
        }
    }
}

impl From<CliDeviceSelector> for FuncsDeviceSelector {
    fn from(value: CliDeviceSelector) -> Self {
        match value {
            CliDeviceSelector {
                devices: None,
                indices: None,
            } => FuncsDeviceSelector::All,
            CliDeviceSelector {
                devices: Some(devices),
                indices: None,
            } => FuncsDeviceSelector::ByName(Arc::<HashSet<String>>::new(
                devices.into_iter().collect(),
            )),
            CliDeviceSelector {
                devices: None,
                indices: Some(indices),
            } => FuncsDeviceSelector::ByIndex(indices.into_iter().collect()),
            CliDeviceSelector { .. } => unreachable!("The variables are mutually exclusive"),
        }
    }
}

impl From<CliCommand> for FuncsCommand {
    fn from(value: CliCommand) -> Self {
        match value {
            CliCommand::Get { selector, output } => FuncsCommand::BrightnessCommand {
                command: BrightnessCommand::Get,
                selector: selector.into(),
                output: output.into(),
            },
            CliCommand::Set {
                percent,
                selector,
                output,
            } => FuncsCommand::BrightnessCommand {
                command: BrightnessCommand::Set { percent },
                selector: selector.into(),
                output: output.into(),
            },
            CliCommand::Inc {
                percent,
                selector,
                output,
            } => FuncsCommand::BrightnessCommand {
                command: BrightnessCommand::Inc { percent },
                selector: selector.into(),
                output: output.into(),
            },
            CliCommand::Dec {
                percent,
                selector,
                output,
            } => FuncsCommand::BrightnessCommand {
                command: BrightnessCommand::Dec { percent },
                selector: selector.into(),
                output: output.into(),
            },
            CliCommand::Max { selector, output } => FuncsCommand::BrightnessCommand {
                command: BrightnessCommand::Max,
                selector: selector.into(),
                output: output.into(),
            },
            CliCommand::Min { selector, output } => FuncsCommand::BrightnessCommand {
                command: BrightnessCommand::Min,
                selector: selector.into(),
                output: output.into(),
            },
            CliCommand::List => FuncsCommand::List,
        }
    }
}

fn main() -> Result<(), Error> {
    let cli = Cli::parse();
    executor::block_on(FuncsCommand::from(cli.command).handle())
}
