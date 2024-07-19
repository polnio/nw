use super::errors::{abort, print_error};
use super::flake::metadata::{FlakeMetadata, FlakeMetadataLocksNodesOriginal};
use crate::utils::xdg::XDG_DIRS;
use anyhow::{Context, Result};
use nw_derive::Optional;
use serde::Deserialize;
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
    pub interactive_shell: String,
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
    pub locked_channel: bool,
    pub os_flake: String,
}
impl ConfigNix {
    fn default_channel(os_flake: &str, locked_channel: bool) -> String {
        let channel: Result<String> = try {
            let mut metadata =
                FlakeMetadata::get(Some(os_flake)).context("Failed to fetch flake metadata")?;
            let channel = metadata
                .locks
                .nodes
                .remove("nixpkgs")
                .and_then(|node| {
                    if locked_channel {
                        node.locked.and_then(|locked| locked.rev)
                    } else {
                        match node.original {
                            Some(FlakeMetadataLocksNodesOriginal::Github(original)) => {
                                original.r#ref
                            }
                            _ => None,
                        }
                    }
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
        let (shell, interactive_shell) = value.general.map_or_else(Default::default, |g| {
            let shell = g.shell.unwrap_or_else(ConfigGeneral::default_shell);
            let interactive_shell = g.interactive_shell.unwrap_or_else(|| shell.clone());
            (shell, interactive_shell)
        });

        let (channel, locked_channel, os_flake) = value.nix.map_or_else(Default::default, |n| {
            let os_flake = n.os_flake.unwrap_or_else(ConfigNix::default_os_flake);
            let locked_channel = n.locked_channel.unwrap_or(true);
            let channel = n
                .channel
                .unwrap_or_else(|| ConfigNix::default_channel(&os_flake, locked_channel));
            (channel, locked_channel, os_flake)
        });

        Self {
            general: ConfigGeneral {
                shell,
                interactive_shell,
            },
            nix: ConfigNix {
                channel,
                locked_channel,
                os_flake,
            },
        }
    }
}

impl Config {
    fn new() -> Result<Self> {
        let Some(config_path) = XDG_DIRS.find_config_file("config.toml") else {
            return Ok(ConfigInternal::default().into());
        };

        let content = std::fs::read_to_string(config_path).context("Failed to open config file")?;
        let config: ConfigInternal =
            toml::from_str(&content).context("Failed to parse config file")?;
        Ok(config.into())
    }
}
