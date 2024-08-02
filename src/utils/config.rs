#[cfg(feature = "ui")]
use super::args::ARGS;
use super::errors::{abort, print_error};
use super::flake::metadata::{FlakeMetadata, FlakeMetadataLocksNodesOriginal};
use crate::utils::xdg::XDG_DIRS;
use anyhow::{Context, Result};
use serde::{Deserialize, Deserializer};
use std::ops::Deref;
use std::sync::LazyLock;

pub struct OnceLock<T> {
    value: std::sync::OnceLock<T>,
}
impl<'de, T: Deserialize<'de>> Deserialize<'de> for OnceLock<T> {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = Option::<T>::deserialize(deserializer)?;
        let lock = std::sync::OnceLock::new();
        if let Some(value) = value {
            let _ = lock.set(value);
        }
        Ok(OnceLock { value: lock })
    }
}
impl<T> Deref for OnceLock<T> {
    type Target = std::sync::OnceLock<T>;
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}
impl<T> Default for OnceLock<T> {
    fn default() -> Self {
        Self {
            value: std::sync::OnceLock::new(),
        }
    }
}

pub static CONFIG: LazyLock<Config> = LazyLock::new(|| match Config::new() {
    Ok(config) => config,
    Err(err) => abort(err),
});

#[derive(Deserialize, Default)]
pub struct Config {
    general: OnceLock<ConfigGeneral>,
    nix: OnceLock<ConfigNix>,
}
impl Config {
    pub fn general(&self) -> &ConfigGeneral {
        self.general.get_or_init(ConfigGeneral::default)
    }
    pub fn nix(&self) -> &ConfigNix {
        self.nix.get_or_init(ConfigNix::default)
    }
}

#[derive(Deserialize, Default)]
pub struct ConfigGeneral {
    shell: OnceLock<String>,
    interactive_shell: OnceLock<String>,
    #[cfg(feature = "ui")]
    ui: OnceLock<bool>,
}
impl ConfigGeneral {
    pub fn shell(&self) -> &String {
        self.shell
            .get_or_init(|| std::option_env!("SHELL").unwrap_or("bash").into())
    }
    pub fn interactive_shell(&self) -> &String {
        self.interactive_shell.get_or_init(|| self.shell().into())
    }
    #[cfg(feature = "ui")]
    pub fn ui(&self) -> bool {
        *self.ui.get_or_init(|| ARGS.ui)
    }
    #[cfg(not(feature = "ui"))]
    pub fn ui(&self) -> bool {
        false
    }
}
#[derive(Deserialize, Default)]
pub struct ConfigNix {
    channel: OnceLock<String>,
    locked_channel: OnceLock<bool>,
    os_flake: OnceLock<String>,
}
impl ConfigNix {
    pub fn channel(&self) -> &String {
        self.channel.get_or_init(|| {
            let channel: Result<String> = try {
                let mut metadata = FlakeMetadata::get(Some(self.os_flake()))
                    .context("Failed to fetch flake metadata")?;
                let channel = metadata
                    .locks
                    .nodes
                    .remove("nixpkgs")
                    .and_then(|node| {
                        if self.locked_channel() {
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
        })
    }

    pub fn locked_channel(&self) -> bool {
        *self.locked_channel.get_or_init(|| true)
    }

    pub fn os_flake(&self) -> &String {
        self.os_flake.get_or_init(|| "/etc/nixos".into())
    }
}

impl Config {
    fn new() -> Result<Self> {
        let Some(config_path) = XDG_DIRS.find_config_file("config.toml") else {
            return Ok(Config::default());
        };

        let content = std::fs::read_to_string(config_path).context("Failed to open config file")?;
        let config: Config = toml::from_str(&content).context("Failed to parse config file")?;
        Ok(config)
    }
}
