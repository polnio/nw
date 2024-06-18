use crate::utils::xdg::XDG_DIRS;
use anyhow::{Context, Error, Result};
use nw_derive::Optional;
use serde::Deserialize;
use std::fs::File;
use std::process::{Command, Stdio};
use std::sync::LazyLock;

use super::errors::abort;
use super::flake::FlakeMetadata;

pub static CONFIG: LazyLock<Config> = LazyLock::new(|| match Config::new() {
    Ok(config) => config,
    Err(err) => abort(err),
});

#[derive(Optional)]
#[derives(Deserialize)]
pub struct Config {
    #[internal]
    pub general: ConfigGeneral,
    #[internal]
    pub nix: ConfigNix,
}
#[derive(Optional)]
#[derives(Deserialize)]
pub struct ConfigGeneral {
    pub shell: String,
}
impl ConfigGeneral {
    fn default_shell() -> String {
        std::option_env!("SHELL").unwrap_or("bash").into()
    }
}
#[derive(Optional)]
#[derives(Deserialize)]
pub struct ConfigNix {
    pub channel: String,
    pub flake: String,
}
impl ConfigNix {
    fn default_channel(flake: &str) -> String {
        let flake: Result<String> = try {
            let stdout = Command::new("nix")
                .args(["flake", "metadata", "--json", flake])
                .stdout(Stdio::piped())
                .spawn()
                .map_err(Error::from)
                .and_then(|mut child| {
                    child.wait()?;
                    child.stdout.context("Failed to retrieve stdout")
                })
                .context("Failed to get nixos flake metadata")?;

            let mut metadata: FlakeMetadata =
                serde_json::from_reader(stdout).context("Failed to parse nixos flake metadata")?;

            let channel = metadata
                .locks
                .nodes
                .remove("nixpkgs")
                .and_then(|node| node.original)
                .and_then(|original| original.r#ref)
                .context("Failed to find nixpkgs chanel")?;
            channel
        };
        flake.unwrap_or("nixos-unstable".into())
    }

    fn default_flake() -> String {
        String::new()
    }
}

impl From<ConfigInternal> for Config {
    fn from(value: ConfigInternal) -> Self {
        let shell = value
            .general
            .and_then(|g| g.shell)
            .unwrap_or_else(ConfigGeneral::default_shell);

        let [channel, flake] = value.nix.map_or([None, None], |n| [n.channel, n.flake]);
        let flake = flake.unwrap_or_else(ConfigNix::default_flake);
        let channel = channel.unwrap_or_else(|| ConfigNix::default_channel(&flake));

        let general = ConfigGeneral { shell };
        let nix = ConfigNix {
            channel,
            flake: String::new(),
        };

        Self { general, nix }
    }
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
            .context("Failed to create config file")?;

        let content = std::fs::read_to_string(config_path).context("Failed to open config file")?;
        let config: ConfigInternal =
            toml::from_str(&content).context("Failed to parse config file")?;
        Ok(config.into())
    }
}
