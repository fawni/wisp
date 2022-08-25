use lazy_static::lazy_static;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct Config {
    pub prefix: String,
    pub accent_color: String,
}

pub fn load(file: &str) -> color_eyre::Result<Config> {
    let config: Config = toml::from_str(std::fs::read_to_string(file)?.as_str())?;
    Ok(config)
}

lazy_static! {
    static ref CONFIG: Config = load("config.toml").expect("failed to load config.toml");
    pub static ref PREFIX: &'static str = &CONFIG.prefix;
    pub static ref ACCENT_COLOR: u32 =
        u32::from_str_radix(CONFIG.accent_color.trim_start_matches("0x"), 16).unwrap();
}
