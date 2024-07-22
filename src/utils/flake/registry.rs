use crate::utils::args::ARGS;
use anyhow::{Context as _, Error, Result};
use std::io::{BufRead as _, BufReader};
use std::str::FromStr;
use subprocess::{Exec, NullFile};

pub struct FlakeRegistry {
    pub id: String,
    pub path: String,
    pub owner: FlakeRegistryOwner,
}

pub enum FlakeRegistryOwner {
    User,
    Global,
}

impl FromStr for FlakeRegistryOwner {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        match s {
            "user" => Ok(FlakeRegistryOwner::User),
            "global" => Ok(FlakeRegistryOwner::Global),
            _ => Err(Error::msg(format!("Invalid flake registry owner: {}", s))),
        }
    }
}

impl FlakeRegistry {
    pub fn get_all() -> Result<Vec<FlakeRegistry>> {
        let mut command = Exec::cmd("nix").args(&["registry", "list"]);
        if ARGS.quiet {
            command = command.arg("--quiet").stderr(NullFile);
        }
        let stdout = command
            .stream_stdout()
            .context("Failed to run `nix registry list`")?;

        let registries = BufReader::new(stdout)
            .lines()
            .filter_map(|line| {
                let line = line.ok()?;
                let mut parts = line.split_whitespace();
                let owner = parts.next()?;
                let id = parts.next()?;
                let path = parts.next()?;
                Some(FlakeRegistry {
                    id: id.to_string(),
                    path: path.to_string(),
                    owner: owner.parse().ok()?,
                })
            })
            .collect();

        Ok(registries)
    }

    pub fn get(id: &str) -> Result<Option<Self>> {
        let registries = FlakeRegistry::get_all()?;
        let registry = registries.into_iter().find(|registry| registry.id == id);
        Ok(registry)
    }
}
