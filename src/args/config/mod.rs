pub mod backups;

pub mod registry;

use clap::Subcommand;

use crate::args::config::{backups::BackupsCommand, registry::RegistryCommand};

#[derive(Subcommand, Debug)]
pub enum ConfigCommand {
    /// Manage backup configuration
    Backups {
        #[clap(subcommand)]
        command: BackupsCommand,
    },
    /// Manage registry
    Registry {
        #[clap(subcommand)]
        command: RegistryCommand,
    },
}
