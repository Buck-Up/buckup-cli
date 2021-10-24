use std::{env, error::Error, path::Path};

use clap::{crate_authors, crate_version, App, Arg, SubCommand};
use smartsync_core::{config, Backup};

fn main() -> Result<(), Box<dyn Error>> {
    let matches = App::new("Smart Sync CLI")
        .version(crate_version!())
        .author(crate_authors!())
        .subcommand(
            SubCommand::with_name("config")
                .about("manage configuration")
                .subcommand(SubCommand::with_name("init").about("initialize configuration"))
                .subcommand(
                    SubCommand::with_name("list-backups").about("list backups in configuration"),
                )
                .subcommand(
                    SubCommand::with_name("add-backup")
                        .about("add backup to configuration")
                        .arg(Arg::with_name("name").required(true).help("name of backup"))
                        .arg(
                            Arg::with_name("dest")
                                .required(true)
                                .help("destination path"),
                        )
                        .arg(
                            Arg::with_name("source")
                                .required(true)
                                .multiple(true)
                                .help("source path(s)"),
                        ),
                )
                .subcommand(
                    SubCommand::with_name("edit-backup")
                        .about("update backup configuration")
                        .subcommand(
                            SubCommand::with_name("rename")
                                .about("update name of backup")
                                .arg(
                                    Arg::with_name("name")
                                        .required(true)
                                        .help("name of backup to edit"),
                                )
                                .arg(
                                    Arg::with_name("new_name")
                                        .required(true)
                                        .help("new name for backup"),
                                ),
                        )
                        .subcommand(
                            SubCommand::with_name("set-dest")
                                .about("set new dest for backup")
                                .arg(
                                    Arg::with_name("name")
                                        .required(true)
                                        .help("name of backup to edit"),
                                )
                                .arg(
                                    Arg::with_name("dest")
                                        .required(true)
                                        .help("new destination path for backup"),
                                ),
                        )
                        .subcommand(
                            SubCommand::with_name("add-source")
                                .about("add new source to backup")
                                .arg(
                                    Arg::with_name("name")
                                        .required(true)
                                        .help("name of backup to edit"),
                                )
                                .arg(
                                    Arg::with_name("source")
                                        .required(true)
                                        .help("new source path for backup"),
                                ),
                        ),
                ),
        )
        .get_matches();

    match matches.subcommand() {
        ("config", Some(config_cmd)) => {
            match config_cmd.subcommand() {
                ("init", Some(_)) => {
                    config::initialize_config()?;
                }
                ("list-backups", Some(_)) => {
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
                ("add-backup", Some(add_backup)) => {
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
                ("edit-backup", Some(edit_backup)) => {
                    match edit_backup.subcommand() {
                        ("rename", Some(rename)) => {
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
                        ("set-dest", Some(set_dest)) => {
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
                        ("add-source", Some(add_source)) => {
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
                            println!("{}", edit_backup.usage());
                        }
                    }
                }
                _ => {
                    println!("{}", config_cmd.usage());
                }
            }
        }
        _ => {
            println!("{}", matches.usage());
        }
    }

    Ok(())
}
