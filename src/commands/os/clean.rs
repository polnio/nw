use crate::utils::args::{OsCleanArgs, ARGS};
use crate::utils::nixos;
use anyhow::{bail, Context as _, Result};
use subprocess::{Exec, NullFile};

pub fn clean(args: &OsCleanArgs) -> Result<()> {
    let mut command = Exec::cmd("sudo").args(&["nix-collect-garbage", "-d"]);
    if ARGS.quiet {
        command = command.arg("--quiet").stdout(NullFile).stderr(NullFile);
    }

    let status = command
        .join()
        .context("Failed to clean NixOS configuration")?;

    if !status.success() {
        bail!("Failed to clean NixOS configuration");
    }

    if args.bootloader {
        nixos::Builder::new()
            .bootloader()
            .build()
            .context("Failed to add to bootloader")?;
    }

    Ok(())
}
