use crate::utils::args::{ShellArgs, ARGS};
use crate::utils::config::CONFIG;
use crate::utils::parse_package_name;
use anyhow::{Context as _, Result};
use std::process::Command;

pub fn shell(args: &ShellArgs) -> Result<()> {
    let packages = args.packages.iter().map(parse_package_name);
    let subcommand = args.command.as_ref().unwrap_or(&CONFIG.general.shell);
    let mut command = Command::new("nix");
    command.arg("shell");
    if ARGS.offline {
        command.arg("--offline");
    }
    if ARGS.quiet {
        command.arg("--quiet");
    }
    command.args(packages);
    command.args(["-c", subcommand]);
    command
        .spawn()
        .and_then(|mut child| child.wait())
        .context("Failed to run nix shell")?;

    Ok(())
}
