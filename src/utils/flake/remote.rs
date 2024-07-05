use anyhow::{Context as _, Error, Result};
use serde::Deserialize;
use std::process::{Command, Stdio};

use crate::utils::args::ARGS;

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FlakeRemote {
    pub hash: String,
    pub store_path: String,
}

impl FlakeRemote {
    pub fn get(flake: &str) -> Result<Self> {
        let mut command = Command::new("nix");
        command.args(["flake", "prefetch", "--json", flake]);
        if ARGS.quiet {
            command.arg("--quiet");
            command.stderr(Stdio::null());
        }
        let stdout = command
            .stdout(Stdio::piped())
            .spawn()
            .map_err(Error::from)
            .and_then(|mut child| {
                child.wait()?;
                child.stdout.context("Failed to retrieve stdout")
            })
            .context("Failed to run `flake metadata`")?;

        let metadata: Self =
            serde_json::from_reader(stdout).context("Failed to parse remote flake")?;
        Ok(metadata)
    }
}
