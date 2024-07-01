use anyhow::{Context as _, Error, Result};
use serde::Deserialize;
use std::io::{BufRead as _, BufReader};
use std::process::{Command, Stdio};

use crate::utils::args::ARGS;

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FlakeRemote {
    pub hash: String,
    pub store_path: String,
}

impl FlakeRemote {
    pub fn get_url(url: &str) -> Result<Self> {
        let mut command = Command::new("nix-prefetch-url");
        command.args(["--print-path", url]);
        if ARGS.quiet {
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

        let mut lines = BufReader::new(stdout).lines();
        let hash = lines.next();
        let store_path = lines.next();
        let (hash, store_path) = hash
            .zip(store_path)
            .context("Failed to parse remote flake")?;
        let metadata = Self {
            hash: hash.unwrap_or_default(),
            store_path: store_path.unwrap_or_default(),
        };
        Ok(metadata)
    }
    pub fn get_flake(flake: &str) -> Result<Self> {
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
