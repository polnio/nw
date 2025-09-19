use crate::utils::args::RunArgs;
use crate::utils::config::CONFIG;
use crate::utils::{build, parse_package_name};
use anyhow::{Context as _, Result};
use subprocess::Exec;

pub fn run(args: &RunArgs) -> Result<()> {
    let is_ui_enabled = CONFIG.general().ui();
    let package = parse_package_name(&args.package);

    let mut command = if is_ui_enabled {
        Exec::cmd("nom").args(&["build", "--print-build-logs", "--no-link", &package])
    } else {
        Exec::cmd("nix").args(&["run", &package])
    };

    command = build::append_args(command);

    if !is_ui_enabled && !args.args.is_empty() {
        command = command.arg("--").args(&args.args);
    }
    command.join().context("Failed to run `nix run`")?;

    if is_ui_enabled {
        let mut command = Exec::cmd("nix").args(&["run", &package]);
        command = build::append_args(command);
        if !args.args.is_empty() {
            command = command.arg("--").args(&args.args);
        }
        command.join().context("Failed to run `nix run`")?;
    }

    Ok(())
}
