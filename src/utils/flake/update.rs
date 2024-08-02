use crate::utils::args::ARGS;
#[cfg(feature = "ui")]
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

    #[cfg(feature = "ui")]
    (if CONFIG.general().ui() {
        (command
            .args(&["--log-format", "internal-json"])
            .stdout(Redirection::Pipe)
            .stderr(Redirection::Merge)
            | Exec::cmd("nom").arg("--json"))
        .join()
    } else {
        command.join()
    })
    .context("Failed to run `nix flake update`")?;
    #[cfg(not(feature = "ui"))]
    command.join().context("Failed to run `nix flake update`")?;

    Ok(())
}
