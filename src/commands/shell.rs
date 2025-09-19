use crate::utils::args::ShellArgs;
use crate::utils::config::CONFIG;
use crate::utils::{build, parse_package_name};
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
        .as_deref()
        .unwrap_or_else(|| CONFIG.general().interactive_shell());

    let mut command = Exec::cmd(if CONFIG.general().ui() { "nom" } else { "nix" });

    if args.dev {
        command = command.arg("develop")
    } else {
        command = command.arg("shell");
    }

    command = build::append_args(command);

    command
        .args(&packages)
        .args(&["-c", CONFIG.general().shell(), "-c", subcommand])
        .join()
        .context("Failed to run nix shell")?;

    Ok(())
}
