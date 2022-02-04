use std::{error::Error, path::Path};

use clap::{Parser, Subcommand};
use smartsync_core::{config, registry, DeviceConfig, FileSync};

type SmartSyncResult = Result<(), Box<dyn Error>>;

#[derive(Parser, Debug)]
#[clap(author, version)]
struct Args {
    #[clap(subcommand)]
    command: Command,
}

#[derive(Subcommand, Debug)]
enum Command {
    /// Manage configuration
    Config {
        #[clap(subcommand)]
        command: ConfigCommand,
    },
}

#[derive(Subcommand, Debug)]
enum ConfigCommand {
    /// Manage backup configuration
    Backups {
        #[clap(subcommand)]
        command: BackupsConfigCommand,
    },
    /// Manage registry
    Registry {
        #[clap(subcommand)]
        command: RegistryConfigCommand,
    },
}

#[derive(Subcommand, Debug)]
enum BackupsConfigCommand {
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
        command: EditBackupConfigCommand,
    },
}

#[derive(Subcommand, Debug)]
enum EditBackupConfigCommand {
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

#[derive(Subcommand, Debug)]
enum RegistryConfigCommand {
    /// Initialize registry
    Init,
    /// List backups in registry
    ListBackups,
    /// Add backup to registry
    AddBackup {
        /// Path to backup
        path: String,
    },
}

fn main() -> SmartSyncResult {
    let args = Args::parse();

    match args.command {
        Command::Config { command } => match command {
            ConfigCommand::Backups { command } => match command {
                BackupsConfigCommand::ListDevices { path } => {
                    list_devices(&path)?;
                }
                BackupsConfigCommand::Init { path } => {
                    config::initialize_config(Path::new(&path))?;
                }
                BackupsConfigCommand::AddDevice { path, device } => {
                    add_device(&path, &device)?;
                }
                BackupsConfigCommand::AddBackup {
                    path,
                    device,
                    name,
                    dest,
                    sources,
                } => {
                    add_device_backup(&path, &device, &name, sources, &dest)?;
                }
                BackupsConfigCommand::EditBackup { command } => match command {
                    EditBackupConfigCommand::RenameBackup {
                        path,
                        device,
                        name,
                        new_name,
                    } => {
                        rename_backup(&path, &device, &name, &new_name)?;
                    }
                    EditBackupConfigCommand::SetDest {
                        path,
                        device,
                        name,
                        dest,
                    } => {
                        set_backup_dest(&path, &device, &name, &dest)?;
                    }
                    EditBackupConfigCommand::AddSource {
                        path,
                        device,
                        name,
                        source,
                    } => {
                        add_backup_source(&path, &device, &name, &source)?;
                    }
                },
            },
            ConfigCommand::Registry { command } => match command {
                RegistryConfigCommand::Init => {
                    registry::initialize_registry()?;
                }
                RegistryConfigCommand::ListBackups => {
                    let reg = registry::load_registry()?;
                    println!("{}", reg);
                }
                RegistryConfigCommand::AddBackup { path } => {
                    register_backup(&path)?;
                }
            },
        },
    }

    Ok(())
}

fn register_backup(backup_path: &str) -> SmartSyncResult {
    let mut reg = registry::load_registry()?;

    reg.add_backup(Path::new(backup_path).to_path_buf());

    registry::save_registry(&reg)?;

    Ok(())
}

fn list_devices(backup_path: &str) -> SmartSyncResult {
    let config = config::load_config(Path::new(backup_path))?;

    println!(
        "{}",
        config
            .devices
            .iter()
            .map(|d| format!("{}", d))
            .collect::<Vec<String>>()
            .join("\n\n")
    );

    Ok(())
}

fn add_device(backup_path: &str, name: &str) -> SmartSyncResult {
    let mut config = config::load_config(Path::new(backup_path))?;

    let device = DeviceConfig::new(name.to_owned());
    config.add_device(device);
    config::save_config(&config, Path::new(backup_path))?;

    Ok(())
}

fn add_device_backup(
    backup_path: &str,
    device: &str,
    name: &str,
    sources: Vec<String>,
    dest: &str,
) -> SmartSyncResult {
    let mut config = config::load_config(Path::new(backup_path))?;

    let mut device_matched = false;
    for device_config in &mut config.devices {
        if device_config.name == device {
            device_matched = true;
            let mut sync = FileSync::new(name.to_owned(), Path::new(dest).to_path_buf());
            for s in sources {
                sync.add_source(Path::new(&s).to_path_buf());
            }
            device_config.add_sync(sync);
            break;
        }
    }

    if !device_matched {
        return Err(format!("no device named {} found", device).into());
    }

    config::save_config(&config, Path::new(backup_path))?;

    Ok(())
}

fn rename_backup(backup_path: &str, device: &str, name: &str, new_name: &str) -> SmartSyncResult {
    let mut config = config::load_config(Path::new(backup_path))?;

    let mut device_matched = false;
    let mut backup_matched = false;
    for device_config in &mut config.devices {
        if device_config.name == device {
            device_matched = true;

            for backup in &mut device_config.files {
                if backup.name == name {
                    backup_matched = true;
                    backup.name = new_name.to_owned();
                    break;
                }
            }
        }
    }

    if !device_matched {
        return Err(format!("no device named {} found", device).into());
    }
    if !backup_matched {
        return Err(format!("no backup named {} found", name).into());
    }

    config::save_config(&config, Path::new(backup_path))?;

    Ok(())
}

fn set_backup_dest(backup_path: &str, device: &str, name: &str, dest: &str) -> SmartSyncResult {
    let mut config = config::load_config(Path::new(backup_path))?;

    let mut device_matched = false;
    let mut backup_matched = false;
    for device_config in &mut config.devices {
        if device_config.name == device {
            device_matched = true;

            for backup in &mut device_config.files {
                if backup.name == name {
                    backup_matched = true;
                    backup.dest = Path::new(dest).to_path_buf();
                    break;
                }
            }
        }
    }

    if !device_matched {
        return Err(format!("no device named {} found", device).into());
    }
    if !backup_matched {
        return Err(format!("no backup named {} found", name).into());
    }

    config::save_config(&config, Path::new(backup_path))?;

    Ok(())
}

fn add_backup_source(backup_path: &str, device: &str, name: &str, source: &str) -> SmartSyncResult {
    let mut config = config::load_config(Path::new(backup_path))?;

    let mut device_matched = false;
    let mut backup_matched = false;
    for device_config in &mut config.devices {
        if device_config.name == device {
            device_matched = true;

            for backup in &mut device_config.files {
                if backup.name == name {
                    backup_matched = true;
                    backup.add_source(Path::new(source).to_path_buf());
                    break;
                }
            }
        }
    }

    if !device_matched {
        return Err(format!("no device named {} found", device).into());
    }
    if !backup_matched {
        return Err(format!("no backup named {} found", name).into());
    }

    config::save_config(&config, Path::new(backup_path))?;

    Ok(())
}
