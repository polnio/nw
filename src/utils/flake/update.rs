use crate::utils::args::ARGS;
use anyhow::{Context as _, Result};
use subprocess::{Exec, NullFile};

pub fn update(flake: Option<&str>) -> Result<()> {
    let mut command = Exec::cmd("nix").args(&["flake", "update"]);
    if let Some(flake) = flake {
        command = command.arg(flake);
    }
    if ARGS.quiet {
        command = command.stdout(NullFile).stderr(NullFile);
    }
    command.join().context("Failed to run `nix flake update`")?;
    Ok(())
}
