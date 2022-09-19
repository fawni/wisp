use lazy_static::lazy_static;
use serde::Deserialize;
use std::error::Error;

#[derive(Deserialize)]
pub struct Config {
    pub prefix: String,
    pub accent_color: String,
}

pub fn load(file: &str) -> Result<Config, Box<dyn Error>> {
    let config: Config = toml::from_str(&std::fs::read_to_string(file)?)?;
    Ok(config)
}

lazy_static! {
    static ref CONFIG: Config = load("config.toml").expect("failed to load config.toml");
    pub static ref PREFIX: &'static str = &CONFIG.prefix;
    pub static ref ACCENT_COLOR: u32 = u32::from_str_radix(&CONFIG.accent_color, 16).unwrap();
}
