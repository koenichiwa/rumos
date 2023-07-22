use crate::{MAX_BRIGHTNESS, MIN_BRIGHTNESS};
use clap::{Args, Parser, Subcommand};

const BRIGHTNESS_PERCENT_RANGE: std::ops::RangeInclusive<i64> =
    MIN_BRIGHTNESS as i64..=MAX_BRIGHTNESS as i64;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
pub struct Cli {
    /// Command to execute
    #[command(subcommand)]
    pub command: Command,
}

#[derive(Args, Debug)]
#[group(required = false, multiple = false)]
pub struct DeviceSelector {
    /// Names of devices that should be changed
    #[arg(short, long, value_name = "DEVICES")]
    pub devices: Option<Vec<String>>,
    /// Indices of devices that should be changed
    #[arg(short, long, value_name = "INDICES")]
    pub indices: Option<Vec<usize>>,
}

#[derive(Args, Debug)]
#[group(required = false, multiple = false)]
pub struct BrightnessOutput {
    /// Do not output result to console
    #[arg(short, long)]
    pub quiet: bool,
    /// Print only brightness level(percentage)
    #[arg(short, long)]
    pub percent: bool,
}

#[derive(Debug, Subcommand)]
pub enum Command {
    /// Get brightness level (in percent)
    Get {
        #[command(flatten)]
        selector: DeviceSelector,
        #[command(flatten)]
        output: BrightnessOutput,
    },
    /// Set brightness level (in percent)
    Set {
        #[arg(value_parser = clap::value_parser!(u32).range(BRIGHTNESS_PERCENT_RANGE))]
        percent: u32,
        #[command(flatten)]
        selector: DeviceSelector,
        #[command(flatten)]
        output: BrightnessOutput,
    },
    /// Increase brightness level (in percent)
    Inc {
        #[arg(value_parser = clap::value_parser!(u32).range(BRIGHTNESS_PERCENT_RANGE))]
        percent: u32,
        #[command(flatten)]
        selector: DeviceSelector,
        #[command(flatten)]
        output: BrightnessOutput,
    },
    /// Decrease brightness level (in percent)
    Dec {
        #[arg(value_parser = clap::value_parser!(u32).range(BRIGHTNESS_PERCENT_RANGE))]
        percent: u32,
        #[command(flatten)]
        selector: DeviceSelector,
        #[command(flatten)]
        output: BrightnessOutput,
    },
    /// Set maximum brightness level
    Max {
        #[command(flatten)]
        selector: DeviceSelector,
        #[command(flatten)]
        output: BrightnessOutput,
    },
    /// Set mininum brightness level
    Min {
        #[command(flatten)]
        selector: DeviceSelector,
        #[command(flatten)]
        output: BrightnessOutput,
    },
    /// List the names of all the available devices
    List,
}
