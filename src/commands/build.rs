use crate::utils::args::BuildArgs;
use crate::utils::config::CONFIG;
use crate::utils::{build, parse_package_name};
use anyhow::{Context as _, Result};
use subprocess::Exec;

pub fn build(args: &BuildArgs) -> Result<()> {
    let package = parse_package_name(&args.package);

    let mut command = if CONFIG.general().ui() {
        Exec::cmd("nom").args(&["build", "--print-build-logs", &package])
    } else {
        Exec::cmd("nix").args(&["build", &package])
    };

    command = build::append_args(command);
    command.join().context("Failed to run `nix run`")?;

    Ok(())
}
