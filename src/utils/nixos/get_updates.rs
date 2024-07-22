use crate::utils::config::CONFIG;
use crate::utils::flake;
use anyhow::Result;

pub fn get_updates() -> Result<Vec<String>> {
    flake::get_updates(Some(&CONFIG.nix().os_flake()))
}
