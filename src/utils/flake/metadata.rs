use anyhow::{Context as _, Error, Result};
use serde::Deserialize;
use std::collections::HashMap;
use std::process::{Command, Stdio};

#[derive(Deserialize)]
pub struct FlakeMetadata {
    pub locks: FlakeMetadataLocks,
}

#[derive(Deserialize)]
pub struct FlakeMetadataLocks {
    pub nodes: HashMap<String, FlakeMetadataLocksNode>,
}

#[derive(Deserialize)]
pub struct FlakeMetadataLocksNode {
    pub original: Option<FlakeMetadataLocksNodesOriginal>,
    pub locked: Option<FlakeMetadataLocksNodesLocked>,
}

#[derive(Deserialize)]
#[serde(tag = "type", rename_all = "lowercase")]
pub enum FlakeMetadataLocksNodesOriginal {
    Github(FlakeMetadataLocksNodesOriginalGithub),
    Tarball(FlakeMetadataLocksNodesOriginalTarball),
    File(FlakeMetadataLocksNodesOriginalFile),
}

impl Into<String> for FlakeMetadataLocksNodesOriginal {
    fn into(self) -> String {
        match self {
            FlakeMetadataLocksNodesOriginal::Github(original) => {
                if let Some(ref r#ref) = original.r#ref {
                    format!("github:{}/{}/{}", original.owner, original.repo, r#ref)
                } else {
                    format!("github:{}/{}", original.owner, original.repo)
                }
            }
            FlakeMetadataLocksNodesOriginal::Tarball(original) => original.url,
            FlakeMetadataLocksNodesOriginal::File(original) => original.url,
        }
    }
}

#[derive(Deserialize)]
pub struct FlakeMetadataLocksNodesOriginalGithub {
    pub owner: String,
    pub repo: String,
    pub r#ref: Option<String>,
}

#[derive(Deserialize)]
pub struct FlakeMetadataLocksNodesOriginalTarball {
    pub url: String,
}

#[derive(Deserialize)]
pub struct FlakeMetadataLocksNodesOriginalFile {
    pub url: String,
}

#[derive(Deserialize)]
pub struct FlakeMetadataLocksNodesLocked {
    #[serde(rename = "narHash")]
    pub hash: String,
}

impl FlakeMetadata {
    pub fn get(flake: &str) -> Result<Self> {
        let mut command = Command::new("nix");
        command.args(["flake", "metadata", "--json"]);
        if !flake.is_empty() {
            command.arg(flake);
        }
        let stdout = command
            .stdout(Stdio::piped())
            .spawn()
            .map_err(Error::from)
            .and_then(|mut child| {
                let status = child.wait()?;
                if !status.success() {
                    return Ok(None);
                }
                let stdout = child.stdout.context("Failed to retrieve stdout")?;
                Ok(Some(stdout))
            })
            .context("Failed to run `flake metadata`")?
            .context("Failed to run `flake metadata`")?;

        let metadata: Self =
            serde_json::from_reader(stdout).context("Failed to parse flake metadata")?;
        Ok(metadata)
    }
}
