use crate::utils::xdg::XDG_DIRS;
use anyhow::{anyhow, Result};
use serde::Deserialize;
use std::fs::File;
use std::sync::LazyLock;

pub static CONFIG: LazyLock<Config> = LazyLock::new(|| match Config::new() {
    Ok(config) => config,
    Err(err) => {
        eprintln!("{}", err);
        std::process::exit(1);
    }
});

#[derive(Deserialize)]
pub struct Config {
    pub general: ConfigGeneral,
}
#[derive(Deserialize)]
pub struct ConfigGeneral {
    #[serde(default = "default_shell")]
    pub shell: String,
}

fn default_shell() -> String {
    std::option_env!("SHELL").unwrap_or("bash").into()
}

impl Config {
    fn new() -> Result<Self> {
        let config_path = XDG_DIRS
            .place_config_file("config.toml")
            .and_then(|config_path| {
                if !config_path.exists() {
                    File::create(&config_path)?;
                }
                Ok(config_path)
            })
            .map_err(|err| anyhow!("Failed to create config file: {}", err))?;

        let content = std::fs::read_to_string(config_path)
            .map_err(|err| anyhow!("Failed to open config file: {}", err))?;

        let config: Config = toml::from_str(&content)
            .map_err(|err| anyhow!("Failed to parse config file: {}", err))?;

        Ok(config)
    }
}
