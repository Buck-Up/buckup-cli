use clap::Subcommand;

#[derive(Subcommand, Debug)]
pub enum EditBackupCommand {
    /// Rename backup in configuration
    RenameBackup {
        /// Path to backup
        path: String,
        /// Name of device
        device: String,
        /// Current name of backup
        name: String,
        /// New name to use
        new_name: String,
    },
    /// Set backup destination in configuration
    SetDest {
        /// Path to backup
        path: String,
        /// Name of device
        device: String,
        /// Name of backup to edit
        name: String,
        /// New dest to use
        dest: String,
    },
    /// Add source to backup configuration
    AddSource {
        /// Path to backup
        path: String,
        /// Name of device
        device: String,
        /// Name of backup to edit
        name: String,
        /// New source to add
        source: String,
    },
}
