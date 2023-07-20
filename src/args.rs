use crate::{MIN_BRIGHTNESS, MAX_BRIGHTNESS};
use clap::{Parser, Subcommand};

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
    #[command(subcommand)]
    pub command: ClapCommands,
    #[arg(short, long, value_name = "DEVICES")] 
    /// Names of devices that should be changed
    pub devices: Vec<String>
}


#[derive(Debug, Subcommand)]
pub enum ClapCommands {
    /// Get brightness level (in percent)
    Get { 
        #[arg(short, long)]
        devices: Vec<String>,
    },
    /// Set brightness level (in percent)
    Set { 
        #[arg(value_parser = clap::value_parser!(u32).range(MIN_BRIGHTNESS as i64..=MAX_BRIGHTNESS as i64))] 
        #[arg(short, long)]
        devices: Vec<String>,
    },
    /// Increase brightness level (in percent)
    Inc { 
        #[arg(value_parser = clap::value_parser!(u32).range(MIN_BRIGHTNESS as i64..=MAX_BRIGHTNESS as i64))] 
        percent: u32,
        #[arg(short, long)]
        devices: Vec<String>,
    },
    /// Decrease brightness level (in percent)
    Dec { 
        #[arg(value_parser = clap::value_parser!(u32).range(MIN_BRIGHTNESS as i64..=MAX_BRIGHTNESS as i64))] 
        percent: u32,
        #[arg(short, long)]
        devices: Vec<String>,
    },
    /// Set maximum brightness level
    Max { 
        #[arg(short, long)]
        devices: Vec<String>,
    },
    /// Set mininum brightness level
    Min { 
        #[arg(short, long)]
        devices: Vec<String>,
    },
}


