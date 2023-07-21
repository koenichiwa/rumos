use crate::{MAX_BRIGHTNESS, MIN_BRIGHTNESS};
use clap::{Parser, Subcommand};

const BRIGHTNESS_PERCENT_RANGE: std::ops::RangeInclusive<i64> =
    MIN_BRIGHTNESS as i64..=MAX_BRIGHTNESS as i64;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
pub struct Cli {
    /// Do not output result to console
    #[arg(short, long)]
    pub quiet: bool,
    /// Print only brightness level(percentage)
    #[arg(short, long)]
    pub percent: bool,
    /// Command to execute
    #[command(subcommand)]
    pub command: Command,
    #[arg(short, long, value_name = "DEVICES")]
    /// Names of devices that should be changed
    pub devices: Vec<String>,
}

#[derive(Debug, Subcommand)]
pub enum Command {
    /// Get brightness level (in percent)
    Get {
        #[arg(short, long)]
        devices: Option<Vec<String>>,
    },
    /// Set brightness level (in percent)
    Set {
        #[arg(value_parser = clap::value_parser!(u32).range(BRIGHTNESS_PERCENT_RANGE))]
        percent: u32,
        #[arg(short, long)]
        devices: Option<Vec<String>>,
    },
    /// Increase brightness level (in percent)
    Inc {
        #[arg(value_parser = clap::value_parser!(u32).range(BRIGHTNESS_PERCENT_RANGE))]
        percent: u32,
        #[arg(short, long)]
        devices: Option<Vec<String>>,
    },
    /// Decrease brightness level (in percent)
    Dec {
        #[arg(value_parser = clap::value_parser!(u32).range(BRIGHTNESS_PERCENT_RANGE))]
        percent: u32,
        #[arg(short, long)]
        devices: Option<Vec<String>>,
    },
    /// Set maximum brightness level
    Max {
        #[arg(short, long)]
        devices: Option<Vec<String>>,
    },
    /// Set mininum brightness level
    Min {
        #[arg(short, long)]
        devices: Option<Vec<String>>,
    },
    List,
}
