use clap::Subcommand;

use crate::args::config::ConfigCommand;

#[derive(Subcommand, Debug)]
pub enum Command {
    /// Manage configuration
    Config {
        #[clap(subcommand)]
        command: ConfigCommand,
    },
    /// Run a backup
    Backup {
        /// Path to backup
        path: String,
        /// Name of device
        device: String,
        /// Print files that would be copied and exit
        #[clap(long)]
        dry_run: bool,
    },
}
