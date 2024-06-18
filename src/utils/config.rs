use crate::utils::xdg::XDG_DIRS;
use anyhow::{Context, Error, Result};
use serde::Deserialize;
use smart_default::SmartDefault;
use std::fs::File;
use std::process::{Command, Stdio};
use std::sync::LazyLock;

use super::errors::abort;
use super::flake::FlakeMetadata;

pub static CONFIG: LazyLock<Config> = LazyLock::new(|| match Config::new() {
    Ok(config) => config,
    Err(err) => abort(err),
});

#[derive(Deserialize, Default)]
pub struct Config {
    #[serde(default)]
    pub general: ConfigGeneral,
    #[serde(default)]
    pub nix: ConfigNix,
}
#[derive(Deserialize, SmartDefault)]
pub struct ConfigGeneral {
    #[serde(default = "default_shell")]
    #[default(default_shell())]
    pub shell: String,
}
#[derive(Deserialize, SmartDefault)]
pub struct ConfigNix {
    #[serde(default = "default_nix_channel")]
    #[default(default_nix_channel())]
    pub channel: String,
}

fn default_shell() -> String {
    std::option_env!("SHELL").unwrap_or("bash").into()
}

fn default_nix_channel() -> String {
    /* match try_default_nix_channel() {
        Ok(channel) => channel,
        Err(err) => abort(err),
    } */
    try_default_nix_channel().unwrap_or("nixos-unstable".into())
}

fn try_default_nix_channel() -> Result<String> {
    let stdout = Command::new("nix")
        // TODO: customizable directory
        .args(["flake", "metadata", "--json", "/etc/nixos"])
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

    Ok(channel)
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
        let config: Config = toml::from_str(&content).context("Failed to parse config file")?;
        Ok(config)
    }
}
