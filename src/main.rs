use std::{env, error::Error, path::Path};

use clap::{crate_authors, crate_version, App, Arg};
use smartsync_core::{config, registry, DeviceConfig, FileSync};

type SmartSyncResult = Result<(), Box<dyn Error>>;

fn main() -> SmartSyncResult {
    let mut edit_backup_cmd = App::new("edit-backup")
        .about("edit backup for device")
        .subcommand(
            App::new("rename")
                .about("update name of backup")
                .arg(Arg::new("path").required(true).help("path to backup"))
                .arg(Arg::new("device").required(true).help("name of device"))
                .arg(
                    Arg::new("name")
                        .required(true)
                        .help("name of backup to edit"),
                )
                .arg(
                    Arg::new("new_name")
                        .required(true)
                        .help("new name for backup"),
                ),
        )
        .subcommand(
            App::new("set-dest")
                .about("set new dest for backup")
                .arg(Arg::new("path").required(true).help("path to backup"))
                .arg(Arg::new("device").required(true).help("name of device"))
                .arg(
                    Arg::new("name")
                        .required(true)
                        .help("name of backup to edit"),
                )
                .arg(
                    Arg::new("dest")
                        .required(true)
                        .help("new destination path for backup"),
                ),
        )
        .subcommand(
            App::new("add-source")
                .about("add new source to backup")
                .arg(Arg::new("path").required(true).help("path to backup"))
                .arg(Arg::new("device").required(true).help("name of device"))
                .arg(
                    Arg::new("name")
                        .required(true)
                        .help("name of backup to edit"),
                )
                .arg(
                    Arg::new("source")
                        .required(true)
                        .help("new source path for backup"),
                ),
        );

    let edit_backup_usage = edit_backup_cmd.render_usage();

    let mut backups_cmd = App::new("backups")
        .about("manage backups")
        .subcommand(
            App::new("list-devices")
                .about("list devices in backup")
                .arg(Arg::new("path").required(true).help("path to backup")),
        )
        .subcommand(
            App::new("init")
                .about("initialize a backup")
                .arg(Arg::new("path").required(true).help("path to backup")),
        )
        .subcommand(
            App::new("add-device")
                .about("add device")
                .arg(Arg::new("path").required(true).help("path to backup"))
                .arg(Arg::new("device").required(true).help("name of device")),
        )
        .subcommand(
            App::new("add-backup")
                .about("add backup for device")
                .arg(Arg::new("path").required(true).help("path to backup"))
                .arg(Arg::new("device").required(true).help("name of device"))
                .arg(Arg::new("name").required(true).help("name of backup"))
                .arg(Arg::new("dest").required(true).help("destination path"))
                .arg(
                    Arg::new("source")
                        .required(true)
                        .multiple_occurrences(true)
                        .help("source path(s)"),
                ),
        )
        .subcommand(edit_backup_cmd);

    let backups_usage = backups_cmd.render_usage();

    let mut registry_cmd = App::new("registry")
        .about("manage registry")
        .subcommand(App::new("init").about("initialize registry"))
        .subcommand(App::new("list-backups").about("list backups in registry"))
        .subcommand(
            App::new("add-backup")
                .about("add backup to registry")
                .arg(Arg::new("path").required(true).help("path to backup")),
        );

    let registry_usage = registry_cmd.render_usage();

    let mut config_cmd = App::new("config")
        .about("manage configuration")
        .subcommand(backups_cmd)
        .subcommand(registry_cmd);

    let config_usage = config_cmd.render_usage();

    let mut app = App::new("Smart Sync CLI")
        .version(crate_version!())
        .author(crate_authors!())
        .subcommand(config_cmd);

    let app_usage = app.render_usage();

    let matches = app.get_matches();

    match matches.subcommand() {
        Some(("config", config_matches)) => match config_matches.subcommand() {
            Some(("backups", backups)) => match backups.subcommand() {
                Some(("list-devices", list_devices_matches)) => {
                    let backup_path = list_devices_matches.value_of("path").unwrap();

                    list_devices(backup_path)?;
                }
                Some(("init", init)) => {
                    let backup_path = init.value_of("path").unwrap();

                    config::initialize_config(Path::new(backup_path))?;
                }
                Some(("add-device", add_device_matches)) => {
                    let backup_path = add_device_matches.value_of("path").unwrap();
                    let name = add_device_matches.value_of("device").unwrap();

                    add_device(backup_path, name)?;
                }
                Some(("add-backup", add_backup_matches)) => {
                    let backup_path = add_backup_matches.value_of("path").unwrap();
                    let device = add_backup_matches.value_of("device").unwrap();
                    let name = add_backup_matches.value_of("name").unwrap();
                    let sources = add_backup_matches
                        .values_of("source")
                        .unwrap()
                        .collect::<Vec<_>>();
                    let dest = add_backup_matches.value_of("dest").unwrap();

                    add_device_backup(backup_path, device, name, sources, dest)?;
                }
                Some(("edit-backup", edit_backup)) => match edit_backup.subcommand() {
                    Some(("rename", rename)) => {
                        let backup_path = rename.value_of("path").unwrap();
                        let device = rename.value_of("device").unwrap();
                        let name = rename.value_of("name").unwrap();
                        let new_name = rename.value_of("new_name").unwrap();

                        rename_backup(backup_path, device, name, new_name)?;
                    }
                    Some(("set-dest", set_dest)) => {
                        let backup_path = set_dest.value_of("path").unwrap();
                        let device = set_dest.value_of("device").unwrap();
                        let name = set_dest.value_of("name").unwrap();
                        let dest = set_dest.value_of("dest").unwrap();

                        set_backup_dest(backup_path, device, name, dest)?;
                    }
                    Some(("add-source", add_source)) => {
                        let backup_path = add_source.value_of("path").unwrap();
                        let device = add_source.value_of("device").unwrap();
                        let name = add_source.value_of("name").unwrap();
                        let source = add_source.value_of("source").unwrap();

                        add_backup_source(backup_path, device, name, source)?;
                    }
                    _ => {
                        println!("{}", edit_backup_usage);
                    }
                },
                _ => {
                    println!("{}", backups_usage);
                }
            },
            Some(("registry", registry_matches)) => match registry_matches.subcommand() {
                Some(("init", _)) => {
                    registry::initialize_registry()?;
                }
                Some(("list-backups", _)) => {
                    let reg = registry::load_registry()?;
                    println!("{}", reg);
                }
                Some(("add-backup", add_backup)) => {
                    let backup_path = add_backup.value_of("path").unwrap();

                    register_backup(backup_path)?;
                }
                _ => {
                    println!("{}", registry_usage);
                }
            },
            _ => {
                println!("{}", config_usage);
            }
        },
        _ => {
            println!("{}", app_usage);
        }
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
    sources: Vec<&str>,
    dest: &str,
) -> SmartSyncResult {
    let mut config = config::load_config(Path::new(backup_path))?;

    let mut device_matched = false;
    for device_config in &mut config.devices {
        if device_config.name == device {
            device_matched = true;
            let mut sync = FileSync::new(name.to_owned(), Path::new(dest).to_path_buf());
            for s in sources {
                sync.add_source(Path::new(s).to_path_buf());
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
