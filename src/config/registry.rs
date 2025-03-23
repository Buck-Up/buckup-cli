use std::path::Path;

use smartsync_core::{config, registry, Backup, DeviceConfig};

use crate::result::SmartSyncResult;

pub fn initialize() -> SmartSyncResult {
    registry::initialize_registry()
}

pub fn list_backups() -> SmartSyncResult {
    let reg = registry::load_registry()?;
    println!("{}", reg);
    Ok(())
}

pub fn register_backup(name: &str, backup_path: &str) -> SmartSyncResult {
    let path = Path::new(&backup_path);

    let mut reg = registry::load_registry()?;
    let backup = Backup::new(name.to_owned(), path.to_path_buf());
    reg.add_backup(backup);
    registry::save_registry(&reg)?;

    let mut config = config::load_config(path)?;
    let device = DeviceConfig::new(name.to_owned());
    config.add_device(device);
    config::save_config(&config, path)?;

    Ok(())
}
