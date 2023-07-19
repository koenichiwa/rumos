use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
pub struct Cli {
    /// Do not output result to console
    #[arg(short, long, value_name = "QUET")]
    pub quiet: bool,
    /// Print only brightness level(percentage)
    #[arg(short, long, value_name = "PERCENT")]
    pub percent: bool,
    #[command(subcommand)]
    pub command: Command,
}

#[non_exhaustive]
#[derive(Debug, Subcommand)]
pub enum Commands {
    #[command(subcommand)]
    ChangeBrightness(ChangeBrightnessCommand)
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

#[derive(Debug, Parser)]
pub struct Percent (
    #[arg(value_parser = clap::value_parser!(u32).range(0..=100))]
    u32,
);
