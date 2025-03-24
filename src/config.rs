use std::path::Path;

use smartsync_core::{
    config::{self, Destination},
    error::BackupError,
};

pub fn initialize_config() -> Result<(), Box<BackupError>> {
    config::initialize_config()
}

pub fn list_backups() -> Result<(), Box<BackupError>> {
    let config = config::load_config()?;

    for backup in config.backups {
        println!("{:#?}", backup);
    }

    Ok(())
}

pub fn add_backup(
    name: String,
    source: String,
    device_name: String,
    dest: String,
) -> Result<(), Box<BackupError>> {
    let mut config = config::load_config()?;

    let source_paths = vec![Path::new(&source).into()];
    let destinations = vec![Destination {
        device_name,
        path: Path::new(&dest).into(),
    }];

    config.backups.push(config::Backup {
        name,
        source_paths,
        destinations,
        last_run: None,
    });

    config::save_config(&config)?;

    Ok(())
}

pub fn rename_backup(current_name: String, new_name: String) -> Result<(), Box<BackupError>> {
    let mut config = config::load_config()?;

    let mut found = false;
    for backup in &mut config.backups {
        if backup.name == current_name {
            found = true;
            backup.name = new_name.clone();
        }
    }

    if !found {
        return Err(Box::new(BackupError::BackupNotFound {
            backup_name: current_name,
        }));
    }

    config::save_config(&config)?;

    Ok(())
}

pub fn add_source(backup_name: String, source: String) -> Result<(), Box<BackupError>> {
    let mut config: config::Config = config::load_config()?;

    let mut found = false;
    for backup in &mut config.backups {
        if backup.name == backup_name {
            found = true;
            backup.source_paths.push(Path::new(&source).into());
        }
    }

    if !found {
        return Err(Box::new(BackupError::BackupNotFound { backup_name }));
    }

    config::save_config(&config)?;

    Ok(())
}

pub fn rename_device(
    backup_name: String,
    current_name: String,
    new_name: String,
) -> Result<(), Box<BackupError>> {
    let mut config: config::Config = config::load_config()?;

    let mut found = false;
    for backup in &mut config.backups {
        if backup.name == backup_name {
            for dest in &mut backup.destinations {
                if dest.device_name == current_name {
                    found = true;
                    dest.device_name = new_name.clone();
                }
            }
        }
    }

    if !found {
        return Err(Box::new(BackupError::DeviceNotFound {
            device_name: current_name,
            backup_name,
        }));
    }

    config::save_config(&config)?;

    Ok(())
}

pub fn set_dest(
    backup_name: String,
    device_name: String,
    dest: String,
) -> Result<(), Box<BackupError>> {
    let mut config: config::Config = config::load_config()?;

    let mut found = false;
    for backup in &mut config.backups {
        if backup.name == backup_name {
            for destination in &mut backup.destinations {
                if destination.device_name == device_name {
                    found = true;
                    destination.path = Path::new(&dest).into();
                }
            }
        }
    }

    if !found {
        return Err(Box::new(BackupError::DeviceNotFound {
            device_name,
            backup_name,
        }));
    }

    config::save_config(&config)?;

    Ok(())
}
