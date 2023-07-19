// Executor
use futures::executor;
// Args
mod args;
// Funcs
mod funcs;

use crate::args::{Cli, Commands};
use crate::funcs::change_brightness;
use clap::Parser;

fn main() -> Result<(), brightness::Error> {
    let cli = Cli::parse();
    match &cli.command {
        Commands::ChangeBrightness(selector, command) => 
            executor::block_on(change_brightness(selector, command, cli.quiet, cli.percent)),
        _ => Ok(()),
    }
}
