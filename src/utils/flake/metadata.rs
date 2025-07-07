use super::registry::FlakeRegistry;
use crate::utils::args::ARGS;
use anyhow::{bail, Context as _, Error, Result};
use serde::Deserialize;
use serde_json::Value;
use std::collections::HashMap;
use subprocess::{Exec, NullFile};

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
    pub inputs: Option<HashMap<String, Value>>,
    pub original: Option<FlakeMetadataLocksNodesOriginal>,
    pub locked: Option<FlakeMetadataLocksNodesLocked>,
}

#[derive(Deserialize)]
#[serde(tag = "type", rename_all = "lowercase")]
pub enum FlakeMetadataLocksNodesOriginal {
    Git(FlakeMetadataLocksNodesOriginalGit),
    Github(FlakeMetadataLocksNodesOriginalGithub),
    SourceHut(FlakeMetadataLocksNodesOriginalSourceHut),
    Tarball(FlakeMetadataLocksNodesOriginalTarball),
    File(FlakeMetadataLocksNodesOriginalFile),
    Indirect(FlakeMetadataLocksNodesOriginalIndirect),
}

fn parse_git(provider: &str, owner: &str, repo: &str, r#ref: Option<&str>) -> String {
    if let Some(ref r#ref) = r#ref {
        format!("{}:{}/{}/{}", provider, owner, repo, r#ref)
    } else {
        format!("{}:{}/{}", provider, owner, repo)
    }
}

impl TryInto<String> for FlakeMetadataLocksNodesOriginal {
    type Error = Error;
    fn try_into(self) -> Result<String> {
        match self {
            Self::Git(original) => Ok(format!("git+{}", original.url)),
            Self::Github(original) => Ok(parse_git(
                "github",
                &original.owner,
                &original.repo,
                original.r#ref.as_deref(),
            )),
            Self::SourceHut(original) => Ok(parse_git(
                "sourcehut",
                &original.owner,
                &original.repo,
                original.r#ref.as_deref(),
            )),
            Self::Tarball(original) => Ok(original.url),
            Self::File(original) if original.url.contains("://") => {
                Ok(format!("file+{}", original.url))
            }
            Self::File(original) => Ok(format!("file:{}", original.url)),
            Self::Indirect(original) => {
                let Some(registry) = FlakeRegistry::get(&original.id)? else {
                    bail!("Failed to find flake registry");
                };
                Ok(registry.path)
            }
        }
    }
}

#[derive(Deserialize)]
pub struct FlakeMetadataLocksNodesOriginalGit {
    pub url: String,
}

#[derive(Deserialize)]
pub struct FlakeMetadataLocksNodesOriginalGithub {
    pub owner: String,
    pub repo: String,
    pub r#ref: Option<String>,
}

#[derive(Deserialize)]
pub struct FlakeMetadataLocksNodesOriginalSourceHut {
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
pub struct FlakeMetadataLocksNodesOriginalIndirect {
    pub id: String,
}

#[derive(Deserialize)]
pub struct FlakeMetadataLocksNodesLocked {
    #[serde(rename = "narHash")]
    pub hash: String,
    pub rev: Option<String>,
}

impl FlakeMetadata {
    pub fn get(flake: Option<&str>) -> Result<Self> {
        let mut command = Exec::cmd("nix").args(&["flake", "metadata", "--json"]);
        if let Some(flake) = flake {
            command = command.arg(flake);
        }
        if ARGS.quiet {
            command = command.arg("--quiet").stderr(NullFile);
        }
        let stdout = command
            .stream_stdout()
            .context("Failed to run `nix flake metadata`")?;

        let metadata = serde_json::from_reader(stdout).context("Failed to parse flake metadata")?;
        Ok(metadata)
    }

    pub fn inputs(&self) -> Vec<&str> {
        let Some(root) = self.locks.nodes.get("root") else {
            return Vec::new();
        };

        let Some(inputs) = &root.inputs else {
            return Vec::new();
        };

        let inputs = inputs.keys().map(String::as_str).collect();
        inputs
    }
}
