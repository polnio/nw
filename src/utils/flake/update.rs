use crate::utils::args::ARGS;
use anyhow::{Context as _, Result};
use std::process::{Command, Stdio};

pub fn update(flake: Option<&str>) -> Result<()> {
    let mut command = Command::new("nix");
    command.args(["flake", "update"]);
    if let Some(flake) = flake {
        command.arg(flake);
    }
    if ARGS.quiet {
        command.stdout(Stdio::null());
        command.stderr(Stdio::null());
    }
    command
        .spawn()
        .and_then(|mut child| child.wait())
        .context("Failed to run `nix flake update`")?;

    Ok(())
}
