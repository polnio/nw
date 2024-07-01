use crate::utils::args::{RunArgs, ARGS};
use crate::utils::parse_package_name;
use anyhow::{Context as _, Result};
use std::process::Command;

pub fn run(args: &RunArgs) -> Result<()> {
    let package = parse_package_name(&args.package);
    let mut command = Command::new("nix");
    command.args(["run", &package]);
    if ARGS.offline {
        command.arg("--offline");
    }
    if ARGS.quiet {
        command.arg("--quiet");
    }
    if !args.args.is_empty() {
        command.arg("--").args(&args.args);
    }
    command
        .spawn()
        .and_then(|mut child| child.wait())
        .context("Failed to run `nix run`")?;

    Ok(())
}
