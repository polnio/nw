use anyhow::{Context as _, Error, Result};
use serde::Deserialize;
use std::io::{BufRead as _, BufReader};
use std::process::{Command, Stdio};

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FlakeRemote {
    pub hash: String,
    pub store_path: String,
}

impl FlakeRemote {
    pub fn get_url(url: &str) -> Result<Self> {
        let stdout = Command::new("nix-prefetch-url")
            .args(["--print-path", url])
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
        let stdout = Command::new("nix")
            .args(["flake", "prefetch", "--json", flake])
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
