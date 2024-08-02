use crate::utils::args::ARGS;
use anyhow::{Context as _, Result};
use serde::Deserialize;
use subprocess::{Exec, NullFile};

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FlakeRemote {
    pub hash: String,
    // pub store_path: String,
}

impl FlakeRemote {
    pub fn get(flake: &str) -> Result<Self> {
        let mut command = Exec::cmd("nix").args(&["flake", "prefetch", "--json", flake]);
        if ARGS.quiet {
            command = command.arg("--quiet").stderr(NullFile);
        }
        let stdout = command
            .stream_stdout()
            .context("Failed to run `nix flake prefetch`")?;

        let metadata = serde_json::from_reader(stdout).context("Failed to parse remote flake")?;
        Ok(metadata)
    }
}
