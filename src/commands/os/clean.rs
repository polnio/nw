use crate::utils::args::{OsCleanArgs, ARGS};
use crate::utils::config::CONFIG;
use crate::utils::nixos;
use anyhow::{bail, Context as _, Result};
use subprocess::{Exec, NullFile};

pub fn clean(args: &OsCleanArgs) -> Result<()> {
    let mut command = Exec::cmd("sudo").args(&["nix-collect-garbage", "-d"]);
    if ARGS.quiet {
        command = command.arg("--quiet").stdout(NullFile).stderr(NullFile);
    }
    if let Some(max_tasks) = CONFIG.general().max_tasks() {
        command = command.args(&["--max-jobs", &max_tasks.to_string()]);
    }
    if let Some(max_cores) = CONFIG.general().max_cores() {
        command = command.args(&["--cores", &max_cores.to_string()]);
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
            .quiet()
            .offline()
            .build()
            .context("Failed to add to bootloader")?;
    }

    Ok(())
}
