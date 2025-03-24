use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[clap(author, version)]
pub struct Args {
    #[clap(subcommand)]
    pub command: Command,
}

#[derive(Subcommand, Debug)]
pub enum Command {
    /// Manage configuration
    Config {
        #[clap(subcommand)]
        command: ConfigCommand,
    },
    /// Run a backup
    Run {
        /// Name of backup
        backup_name: String,
        /// Name of device
        device_name: String,
        /// Print files that would be copied and exit
        #[clap(long)]
        dry_run: bool,
    },
}

#[derive(Subcommand, Debug)]
pub enum ConfigCommand {
    Init,
    ListBackups,
    AddBackup {
        name: String,
        source: String,
        device_name: String,
        dest: String,
    },
    EditBackup {
        #[clap(subcommand)]
        command: EditBackupCommand,
    },
}

#[derive(Subcommand, Debug)]
pub enum EditBackupCommand {
    RenameBackup {
        current_name: String,
        new_name: String,
    },
    AddSource {
        backup_name: String,
        source: String,
    },
    RenameDevice {
        backup_name: String,
        current_name: String,
        new_name: String,
    },
    SetDest {
        backup_name: String,
        device_name: String,
        dest: String,
    },
}
