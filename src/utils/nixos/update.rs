use crate::utils::config::CONFIG;
use crate::utils::flake;
use anyhow::Result;

pub fn update() -> Result<()> {
    flake::update(Some(CONFIG.nix().os_flake()))
}
