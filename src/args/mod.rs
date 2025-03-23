pub mod command;
pub mod config;

use clap::Parser;

use crate::args::command::Command;

#[derive(Parser, Debug)]
#[clap(author, version)]
pub struct Args {
    #[clap(subcommand)]
    pub command: Command,
}
