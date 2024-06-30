use super::errors::{abort, print_error};
use super::flake::metadata::{FlakeMetadata, FlakeMetadataLocksNodesOriginal};
use crate::utils::xdg::XDG_DIRS;
use anyhow::{Context, Result};
use nw_derive::Optional;
use serde::Deserialize;
use std::fs::File;
use std::sync::LazyLock;

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
    pub os_flake: String,
}
impl ConfigNix {
    fn default_channel(os_flake: &str) -> String {
        let channel: Result<String> = try {
            let mut metadata =
                FlakeMetadata::get(os_flake).context("Failed to fetch flake metadata")?;
            let channel = metadata
                .locks
                .nodes
                .remove("nixpkgs")
                .and_then(|node| match node.original {
                    Some(FlakeMetadataLocksNodesOriginal::Github(original)) => original.r#ref,
                    _ => None,
                })
                .context("Failed to find nixpkgs chanel")?;
            channel
        };
        match channel.context("Failed to retrieve nixos channel") {
            Ok(channel) => channel,
            Err(err) => {
                print_error(err);
                "nixos-unstable".into()
            }
        }
    }

    fn default_os_flake() -> String {
        "/etc/nixos".into()
    }
}

impl From<ConfigInternal> for Config {
    fn from(value: ConfigInternal) -> Self {
        // General
        let shell = value
            .general
            .and_then(|g| g.shell)
            .unwrap_or_else(ConfigGeneral::default_shell);

        // Nix
        let [channel, os_flake] = value
            .nix
            .map_or_else(Default::default, |n| [n.channel, n.os_flake]);
        let os_flake = os_flake.unwrap_or_else(ConfigNix::default_os_flake);
        let channel = channel.unwrap_or_else(|| ConfigNix::default_channel(&os_flake));

        Self {
            general: ConfigGeneral { shell },
            nix: ConfigNix { channel, os_flake },
        }
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
