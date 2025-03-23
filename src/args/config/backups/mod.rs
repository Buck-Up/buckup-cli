pub mod edit_backup;

use clap::Subcommand;

use crate::args::config::backups::edit_backup::EditBackupCommand;

#[derive(Subcommand, Debug)]
pub enum BackupsCommand {
    /// List devices in backup configuration
    ListDevices {
        /// Path to backup
        path: String,
    },
    /// Initialize backup configuration
    Init {
        /// Path to backup
        path: String,
    },
    /// Add device to backup configuration
    AddDevice {
        /// Path to backup
        path: String,
        /// Name of device
        device: String,
    },
    /// Add backup to configuration
    AddBackup {
        /// Path to backup
        path: String,
        /// Name of device
        device: String,
        /// Name of backup
        name: String,
        /// Destination path
        dest: String,
        /// Source path(s)
        sources: Vec<String>,
    },
    /// Edit backup configuration
    EditBackup {
        #[clap(subcommand)]
        command: EditBackupCommand,
    },
}
