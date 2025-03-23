use clap::Subcommand;

#[derive(Subcommand, Debug)]
pub enum RegistryCommand {
    /// Initialize registry
    Init,
    /// List backups in registry
    ListBackups,
    /// Add backup to registry
    AddBackup {
        /// Name of backup
        name: String,
        /// Path to backup
        path: String,
    },
}
