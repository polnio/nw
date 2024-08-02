use crate::utils::args::ARGS;
use crate::utils::config::CONFIG;
use anyhow::{Context, Result};
use subprocess::{Exec, NullFile, Redirection};

pub struct Builder {
    apply: bool,
    bootloader: bool,
}

impl Builder {
    pub fn new() -> Self {
        Self {
            apply: false,
            bootloader: false,
        }
    }
    pub fn apply(&mut self) -> &Self {
        self.apply = true;
        self
    }
    pub fn bootloader(&mut self) -> &Self {
        self.bootloader = true;
        self
    }

    pub fn build(&self) -> Result<()> {
        let mut command = Exec::cmd("sudo").arg("nixos-rebuild");
        command = match (self.apply, self.bootloader) {
            (false, false) => command.arg("build"),
            (false, true) => command.arg("boot"),
            (true, false) => command.arg("test"),
            (true, true) => command.arg("switch"),
        };

        command = command.args(&["--flake", CONFIG.nix().os_flake()]);

        if ARGS.quiet {
            command = command.stdout(NullFile).stderr(NullFile);
        }

        (if CONFIG.general().ui() {
            // Avoid sudo prompt being captured by nix-output-monitor
            Exec::cmd("sudo")
                .arg("true")
                .join()
                .context("Failed to run sudo")?;
            (command
                .args(&["--log-format", "internal-json"])
                .stdout(Redirection::Pipe)
                .stderr(Redirection::Merge)
                | Exec::cmd("nom").arg("--json"))
            .join()
        } else {
            command.join()
        })
        .context("Failed to build NixOS configuration")?;

        Ok(())
    }
}
