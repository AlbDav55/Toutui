use config::{Config as ConfigLib, File};
use serde::Deserialize;
use color_eyre::eyre::{Result, Report};
use std::path::PathBuf;

#[derive(Debug, Deserialize)]
pub struct ConfigFile {
    pub colors: Colors,
    pub player: Player,
}

#[derive(Debug, Deserialize)]
pub struct Colors {
    pub background_color: Vec<u8>,
    pub log_background_color: Vec<u8>,
    pub header_background_color: Vec<u8>,
    pub line_header_color: Vec<u8>,
    pub list_background_color: Vec<u8>,
    pub list_background_color_alt_row: Vec<u8>,
    pub list_selected_background_color: Vec<u8>,
    pub list_selected_foreground_color: Vec<u8>,
    pub search_bar_foreground_color: Vec<u8>,
    pub login_foreground_color: Vec<u8>,
}

#[derive(Debug, Deserialize)]
pub struct Player {
    pub cvlc: String,
    pub cvlc_term: String,
    pub address: String,
    pub port: String,
}

/// load config from `config.toml` file
pub fn load_config() -> Result<ConfigFile> {
    let mut config_path = dirs::config_dir().unwrap_or_else(|| PathBuf::from("."));
    config_path.push("toutui/config.toml");
    let config_path_str = config_path.to_str().unwrap().to_string();

    let config = ConfigLib::builder()
        .add_source(File::with_name(&config_path_str))
        .build()
        .map_err(|e| Report::new(e))?;

    let colors: Colors = config.get("colors")
        .map_err(|e| Report::new(e))?;
    let player: Player = config.get("player")
        .map_err(|e| Report::new(e))?;

    Ok(ConfigFile { colors, player })
}

