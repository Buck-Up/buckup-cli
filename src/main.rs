use std::{env, error::Error, path::Path};

use clap::{crate_authors, crate_version, App, Arg};
use smartsync_core::{config, Backup};

fn main() -> Result<(), Box<dyn Error>> {
    let add_backup_cmd = App::new("add-backup")
        .about("add backup to configuration")
        .arg(Arg::new("name").required(true).help("name of backup"))
        .arg(Arg::new("dest").required(true).help("destination path"))
        .arg(
            Arg::new("source")
                .required(true)
                .multiple_occurrences(true)
                .help("source path(s)"),
        );

    let mut edit_backup_cmd = App::new("edit-backup")
        .about("update backup configuration")
        .subcommand(
            App::new("rename")
                .about("update name of backup")
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

    let mut config_cmd = App::new("config")
        .about("manage configuration")
        .subcommand(App::new("init").about("initialize configuration"))
        .subcommand(App::new("list-backups").about("list backups in configuration"))
        .subcommand(add_backup_cmd)
        .subcommand(edit_backup_cmd);

    let config_usage = config_cmd.render_usage();

    let mut app = App::new("Smart Sync CLI")
        .version(crate_version!())
        .author(crate_authors!())
        .subcommand(config_cmd);

    let app_usage = app.render_usage();

    let matches = app.get_matches();

    match matches.subcommand() {
        Some(("config", config_matches)) => {
            match config_matches.subcommand() {
                Some(("init", _)) => {
                    config::initialize_config()?;
                }
                Some(("list-backups", _)) => {
                    let config = config::load_config()?;
                    println!(
                        "{}",
                        config
                            .backups
                            .iter()
                            .map(|b| format!("{}", b))
                            .collect::<Vec<String>>()
                            .join("\n-----\n")
                    );
                }
                Some(("add-backup", add_backup)) => {
                    let mut config = config::load_config()?;

                    // all args required so unwrap is safe
                    let name = add_backup.value_of("name").unwrap();
                    let sources = add_backup.values_of("source").unwrap();
                    let dest = add_backup.value_of("dest").unwrap();
                    let mut new_backup =
                        Backup::new(name.to_owned(), Path::new(dest).to_path_buf());
                    for s in sources {
                        new_backup.add_source(Path::new(s).to_path_buf());
                    }

                    config.add_backup(new_backup);

                    config::save_config(&config)?;
                }
                Some(("edit-backup", edit_backup)) => {
                    match edit_backup.subcommand() {
                        Some(("rename", rename)) => {
                            let mut config = config::load_config()?;

                            // all args required so unwrap is safe
                            let name = rename.value_of("name").unwrap();
                            let new_name = rename.value_of("new_name").unwrap();

                            for b in &mut config.backups {
                                if b.name == name {
                                    b.name = new_name.to_string();
                                }
                            }

                            config::save_config(&config)?;
                        }
                        Some(("set-dest", set_dest)) => {
                            let mut config = config::load_config()?;

                            // all args required so unwrap is safe
                            let name = set_dest.value_of("name").unwrap();
                            let dest = set_dest.value_of("dest").unwrap();

                            for b in &mut config.backups {
                                if b.name == name {
                                    b.dest = Path::new(dest).to_path_buf();
                                }
                            }

                            config::save_config(&config)?;
                        }
                        Some(("add-source", add_source)) => {
                            let mut config = config::load_config()?;

                            // all args required so unwrap is safe
                            let name = add_source.value_of("name").unwrap();
                            let source = add_source.value_of("source").unwrap();

                            for b in &mut config.backups {
                                if b.name == name {
                                    b.sources.push(Path::new(source).to_path_buf());
                                }
                            }

                            config::save_config(&config)?;
                        }
                        _ => {
                            println!("{}", edit_backup_usage);
                        }
                    }
                }
                _ => {
                    println!("{}", config_usage);
                }
            }
        }
        _ => {
            println!("{}", app_usage);
        }
    }

    Ok(())
}
