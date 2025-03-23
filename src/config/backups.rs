use std::path::Path;

use smartsync_core::{config, DeviceConfig, FileSync};

use crate::result::SmartSyncResult;

pub fn list_devices(backup_path: &str) -> SmartSyncResult {
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

pub fn add_device(backup_path: &str, name: String) -> SmartSyncResult {
    let mut config = config::load_config(Path::new(backup_path))?;

    let device = DeviceConfig::new(name);
    config.add_device(device);
    config::save_config(&config, Path::new(backup_path))?;

    Ok(())
}

pub fn add_device_backup(
    backup_path: &str,
    device: &str,
    name: String,
    sources: Vec<String>,
    dest: &str,
) -> SmartSyncResult {
    let mut config = config::load_config(Path::new(backup_path))?;

    let mut device_matched = false;
    for device_config in &mut config.devices {
        if device_config.name == device {
            device_matched = true;
            let mut sync = FileSync::new(name, Path::new(dest).to_path_buf());
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

pub fn rename_backup(
    backup_path: &str,
    device: &str,
    name: &str,
    new_name: &str,
) -> SmartSyncResult {
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

pub fn set_backup_dest(backup_path: &str, device: &str, name: &str, dest: &str) -> SmartSyncResult {
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

pub fn add_backup_source(
    backup_path: &str,
    device: &str,
    name: &str,
    source: &str,
) -> SmartSyncResult {
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
