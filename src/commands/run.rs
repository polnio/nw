use crate::utils::args::{RunArgs, ARGS};
#[cfg(feature = "ui")]
use crate::utils::config::CONFIG;
use crate::utils::parse_package_name;
use anyhow::{Context as _, Result};
use subprocess::Exec;

pub fn run(args: &RunArgs) -> Result<()> {
    let package = parse_package_name(&args.package);

    #[cfg(feature = "ui")]
    if CONFIG.general().ui() {
        Exec::cmd("nom")
            .args(&["build", "--print-build-logs", &package])
            .join()
            .context("Failed to build package")?;
    }

    let mut command = Exec::cmd("nix").args(&["run", &package]);
    if ARGS.offline {
        command = command.arg("--offline");
    }
    if ARGS.quiet {
        command = command.arg("--quiet");
    }
    if !args.args.is_empty() {
        command = command.arg("--").args(&args.args);
    }
    command.join().context("Failed to run `nix run`")?;

    Ok(())
}
