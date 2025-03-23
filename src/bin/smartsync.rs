use std::path::Path;

use clap::Parser;
use smartsync_cli::{
    args::{
        command::Command,
        config::{
            backups::{edit_backup::EditBackupCommand, BackupsCommand},
            registry::RegistryCommand,
            ConfigCommand,
        },
        Args,
    },
    config::backups,
    config::registry,
    result::SmartSyncResult,
};
use smartsync_core::{config, runner};

fn main() -> SmartSyncResult {
    let args = Args::parse();

    match args.command {
        Command::Config { command } => match command {
            ConfigCommand::Backups { command } => match command {
                BackupsCommand::ListDevices { path } => {
                    backups::list_devices(&path)?;
                }
                BackupsCommand::Init { path } => {
                    config::initialize_config(Path::new(&path))?;
                }
                BackupsCommand::AddDevice { path, device } => {
                    backups::add_device(&path, device)?;
                }
                BackupsCommand::AddBackup {
                    path,
                    device,
                    name,
                    dest,
                    sources,
                } => {
                    backups::add_device_backup(&path, &device, name, sources, &dest)?;
                }
                BackupsCommand::EditBackup { command } => match command {
                    EditBackupCommand::RenameBackup {
                        path,
                        device,
                        name,
                        new_name,
                    } => {
                        backups::rename_backup(&path, &device, &name, &new_name)?;
                    }
                    EditBackupCommand::SetDest {
                        path,
                        device,
                        name,
                        dest,
                    } => {
                        backups::set_backup_dest(&path, &device, &name, &dest)?;
                    }
                    EditBackupCommand::AddSource {
                        path,
                        device,
                        name,
                        source,
                    } => {
                        backups::add_backup_source(&path, &device, &name, &source)?;
                    }
                },
            },
            ConfigCommand::Registry { command } => match command {
                RegistryCommand::Init => {
                    registry::initialize()?;
                }
                RegistryCommand::ListBackups => {
                    registry::list_backups()?;
                }
                RegistryCommand::AddBackup { name, path } => {
                    registry::register_backup(&name, &path)?;
                }
            },
        },
        Command::Backup {
            path,
            device,
            dry_run,
        } => {
            run_backup(&path, &device, dry_run)?;
        }
    }

    Ok(())
}

fn run_backup(backup_path: &str, device: &str, dry_run: bool) -> SmartSyncResult {
    let path = Path::new(backup_path);
    let config = config::load_config(path)?;

    let mut device_matched = false;
    for device_config in &config.devices {
        if device_config.name == device {
            device_matched = true;

            runner::run_backup(path, device_config, dry_run)?;
        }
    }

    if !device_matched {
        return Err(format!("no device named {} found", device).into());
    }

    Ok(())
}
