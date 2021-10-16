use std::{
    env,
    error::Error,
    fs,
    path::{Path, PathBuf},
};

use clap::{crate_authors, crate_version, App, Arg, SubCommand};
use smartsync_core::{Backup, Config};

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
                        .arg(Arg::with_name("name").required(true))
                        .arg(Arg::with_name("dest").required(true))
                        .arg(Arg::with_name("source").required(true).multiple(true)),
                ),
        )
        .get_matches();

    match matches.subcommand() {
        ("config", Some(config_cmd)) => {
            match config_cmd.subcommand() {
                ("init", Some(_)) => {
                    init_config()?;
                }
                ("list-backups", Some(_)) => {
                    let config = load_config()?;
                    println!("configured backups:");
                    for b in config.backups {
                        println!("{:?}", b);
                    }
                }
                ("add-backup", Some(add_backup)) => {
                    let mut config = load_config()?;

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

                    save_config(&config)?;
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

fn init_config() -> Result<(), Box<dyn Error>> {
    let config = load_config()?;
    save_config(&config)?;

    let contents = toml::to_string(&config)?;
    println!("config initialized at {:?}", config_path());
    println!("{}", contents);

    Ok(())
}

fn config_path() -> PathBuf {
    let home_dir = env::var("HOME").unwrap();
    Path::new(&home_dir).join(".strongbox.toml")
}

fn load_config() -> Result<Config, Box<dyn Error>> {
    let f = config_path();

    if f.exists() {
        let contents = fs::read_to_string(f)?;
        let config = toml::from_str(&contents)?;
        Ok(config)
    } else {
        Ok(Config::default())
    }
}

fn save_config(config: &Config) -> Result<(), Box<dyn Error>> {
    let f = config_path();

    let contents = toml::to_string(config)?;
    fs::write(f, contents)?;

    Ok(())
}
