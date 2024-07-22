use crate::utils::args::{ShellArgs, ARGS};
use crate::utils::config::CONFIG;
use crate::utils::parse_package_name;
use anyhow::{Context as _, Result};
use subprocess::Exec;

pub fn shell(args: &ShellArgs) -> Result<()> {
    let packages = args
        .packages
        .iter()
        .map(parse_package_name)
        .collect::<Vec<_>>();
    let subcommand = args
        .command
        .as_ref()
        .unwrap_or_else(|| CONFIG.general().interactive_shell());

    let mut command = Exec::cmd("nix").args(&["shell"]);
    if ARGS.offline {
        command = command.arg("--offline");
    }
    if ARGS.quiet {
        command = command.arg("--quiet");
    }
    command
        .args(&packages)
        .args(&["-c", CONFIG.general().shell(), "-c", subcommand])
        .join()
        .context("Failed to run nix shell")?;

    Ok(())
}
