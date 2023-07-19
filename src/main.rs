// Executor
use futures::executor;
// Args
mod args;
// Funcs
mod funcs;

use crate::args::{Cli, Commands};
use crate::funcs::change_brightness;
use clap::Parser;

fn main() {
    let cli = Cli::parse();
    match &cli.command {
        Commands::ChangeBrightness(command) => {executor::block_on(change_brightness(command, cli.quiet, cli.percent));},
        _ => {},
    }
}
