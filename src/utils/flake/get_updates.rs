use super::metadata::FlakeMetadata;
use super::remote::FlakeRemote;
use crate::utils::args::ARGS;
use crate::utils::errors;
use anyhow::{Context as _, Result};
use rayon::prelude::*;

pub fn get_updates(flake: Option<&str>) -> Result<Vec<String>> {
    let metadata = FlakeMetadata::get(flake).context("Failed to fetch flake metadata")?;

    let inputs = metadata
        .inputs()
        .into_iter()
        .map(str::to_owned)
        .collect::<Vec<_>>();

    let updates = metadata
        .locks
        .nodes
        .into_par_iter() // Parallel downloads
        .filter_map(|(name, node)| {
            if !inputs.contains(&name) {
                return None;
            }
            let flake: String = node.original?.try_into().ok()?;
            let remote = match FlakeRemote::get(&flake) {
                Ok(remote) => remote,
                Err(e) => {
                    if !ARGS.quiet {
                        errors::print_error(e);
                    }
                    return None;
                }
            };
            (node.locked?.hash != remote.hash).then_some(name)
        })
        .collect::<Vec<_>>();

    Ok(updates)
}
