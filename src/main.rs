use std::{
    env,
    error::Error,
    fs,
    path::{Path, PathBuf},
};

use smartsync_core::Config;

fn main() -> Result<(), Box<dyn Error>> {
    init_config()?;

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
