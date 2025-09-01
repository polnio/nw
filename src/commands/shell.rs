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
        .as_deref()
        .unwrap_or_else(|| CONFIG.general().interactive_shell());

    let mut command = Exec::cmd(if CONFIG.general().ui() { "nom" } else { "nix" });

    if args.dev {
        command = command.arg("develop")
    } else {
        command = command.arg("shell");
    }

    if ARGS.offline {
        command = command.args(&["--offline", "--no-net"])
    }
    if ARGS.quiet {
        command = command.arg("--quiet");
    }
    if let Some(max_tasks) = CONFIG.general().max_tasks() {
        command = command.args(&["--max-jobs", &max_tasks.to_string()]);
    }
    if let Some(max_cores) = CONFIG.general().max_cores() {
        command = command.args(&["--cores", &max_cores.to_string()]);
    }
    command
        .args(&packages)
        .args(&["-c", CONFIG.general().shell(), "-c", subcommand])
        .join()
        .context("Failed to run nix shell")?;

    Ok(())
}
