use crate::utils::args::ARGS;
use crate::utils::config::CONFIG;
use anyhow::{Context, Result};
use std::process::{Command, Stdio};

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
        let mut command = Command::new("sudo");
        command.arg("nixos-rebuild");
        match (self.apply, self.bootloader) {
            (false, false) => command.arg("build"),
            (false, true) => command.arg("boot"),
            (true, false) => command.arg("test"),
            (true, true) => command.arg("switch"),
        };

        command.args(["--flake", &CONFIG.nix.os_flake]);

        if ARGS.quiet {
            command.stdout(Stdio::null());
            command.stderr(Stdio::null());
        }

        command
            .status()
            .context("Failed to build NixOS configuration")?;

        Ok(())
    }
}
