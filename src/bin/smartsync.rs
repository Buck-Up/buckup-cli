use clap::Parser;
use smartsync_cli::{args, config};
use smartsync_core::{error::BackupError, runner};

fn main() -> Result<(), Box<BackupError>> {
    let args = args::Args::parse();

    match args.command {
        args::Command::Config { command } => match command {
            args::ConfigCommand::Init => {
                config::initialize_config()?;
            }
            args::ConfigCommand::ListBackups => {
                config::list_backups()?;
            }
            args::ConfigCommand::AddBackup {
                name,
                source,
                device_name,
                dest,
            } => {
                config::add_backup(name, source, device_name, dest)?;
            }
            args::ConfigCommand::EditBackup { command } => match command {
                args::EditBackupCommand::RenameBackup {
                    current_name,
                    new_name,
                } => {
                    config::rename_backup(current_name, new_name)?;
                }
                args::EditBackupCommand::AddSource {
                    backup_name,
                    source,
                } => {
                    config::add_source(backup_name, source)?;
                }
                args::EditBackupCommand::RenameDevice {
                    backup_name,
                    current_name,
                    new_name,
                } => {
                    config::rename_device(backup_name, current_name, new_name)?;
                }
                args::EditBackupCommand::SetDest {
                    backup_name,
                    device_name,
                    dest,
                } => {
                    config::set_dest(backup_name, device_name, dest)?;
                }
            },
        },
        args::Command::Run {
            backup_name,
            device_name,
            dry_run,
        } => {
            runner::run_backup(&backup_name, &device_name, dry_run)?;
        }
    }

    Ok(())
}
