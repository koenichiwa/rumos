use std::default;

use clap::{Parser, Subcommand, ValueEnum};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
pub struct Cli {
    /// Do not output result to console
    #[arg(short, long, value_name = "QUIET")]
    pub quiet: bool,
    /// Print only brightness level(percentage)
    #[arg(short, long, value_name = "PERCENT")]
    pub percent: bool,
    #[command(subcommand)]
    pub command: Commands,
}


#[derive(Debug, Subcommand, Parser)]
#[non_exhaustive]
pub enum Commands {
    ChangeBrightness{ #[command(flatten)] command: ChangeBrightnessCommand, #[arg(short, long, value_name = "DEVICES")] selector: DeviceSelector }
}

#[derive(Debug, Subcommand)]
pub enum ChangeBrightnessCommand {
    /// Get brightness level (in percent)
    Get,
    /// Set brightness level (in percent)
    Set(Percent),
    /// Increase brightness level (in percent)
    Inc(Percent),
    /// Decrease brightness level (in percent)
    Dec(Percent),
    /// Set maximum brightness level
    Max,
    /// Set mininum brightness level
    Min,
}

#[derive(Default, Debug, ValueEnum, Clone)]
pub enum DeviceSelector {
    #[default]
    All,
    ByName(#[arg(multiple=true)] Vec<String>),
}

#[derive(Debug, Parser)]
pub struct Percent {
    #[arg(value_parser = clap::value_parser!(u32).range(0..=100))]
    pub value: u32,
}
