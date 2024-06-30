use crate::utils::args::{FlakeUpdateArgs, ARGS};
use anyhow::{Context as _, Result};
use std::process::Command;

pub fn update(args: &FlakeUpdateArgs) -> Result<()> {
    if ARGS.offline {
        eprintln!("Offline mode is not supported");
        std::process::exit(1);
    }

    let mut command = Command::new("nix");
    command.args(["flake", "update"]);
    if let Some(flake) = &args.flake {
        command.arg(flake);
    }
    command
        .spawn()
        .and_then(|mut child| child.wait())
        .context("Failed to run `nix flake update`")?;

    Ok(())
}
