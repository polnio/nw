use crate::utils::args::ARGS;
use crate::utils::config::CONFIG;
use anyhow::{Context as _, Result};
use subprocess::{Exec, NullFile, Redirection};

pub fn update(flake: Option<&str>) -> Result<()> {
    let mut command = Exec::cmd("nix").args(&["flake", "update"]);

    if let Some(flake) = flake {
        command = command.arg(flake);
    }

    if ARGS.quiet {
        command = command.stdout(NullFile).stderr(NullFile);
    }

    let result = if CONFIG.general().ui() {
        let command = command
            .args(&["--log-format", "internal-json"])
            .stdout(Redirection::Pipe)
            .stderr(Redirection::Merge)
            | Exec::cmd("nom").arg("--json");
        command.join()
    } else {
        command.join()
    };
    result.context("Failed to run `nix flake update`")?;

    Ok(())
}
