use crate::utils::{
    args::{OsCleanArgs, ARGS},
    nixos,
};
use anyhow::{bail, Context as _, Result};
use std::process::{Command, Stdio};

pub fn clean(args: &OsCleanArgs) -> Result<()> {
    let mut command = Command::new("sudo");
    command.args(["nix-collect-garbage", "-d"]);

    if ARGS.quiet {
        command.arg("--quiet");
        command.stdout(Stdio::null());
        command.stderr(Stdio::null());
    }

    let status = command
        .status()
        .context("Failed to clean NixOS configuration")?;

    if !status.success() {
        bail!("Failed to clean NixOS configuration");
    }

    if args.bootloader {
        nixos::Builder::new().bootloader().build()?;
    }

    Ok(())
}
