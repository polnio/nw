use super::metadata::{FlakeMetadata, FlakeMetadataLocksNodesOriginal};
use super::remote::FlakeRemote;
use crate::utils::args::ARGS;
use crate::utils::errors;
use anyhow::{Context as _, Result};

pub fn get_updates(flake: Option<&str>) -> Result<Vec<String>> {
    let metadata = FlakeMetadata::get(flake).context("Failed to fetch flake metadata")?;

    let inputs = metadata.inputs().into_iter().cloned().collect::<Vec<_>>();

    let handles = metadata
        .locks
        .nodes
        .into_iter()
        .filter_map(|(name, node)| {
            if !inputs.contains(&name) {
                return None;
            }
            let Some(flake) = node.original else {
                return None;
            };
            let is_flake = !matches!(flake, FlakeMetadataLocksNodesOriginal::File(_));
            let flake: String = flake.try_into().ok()?;
            let handle = std::thread::spawn(move || {
                if is_flake {
                    FlakeRemote::get_flake(&flake)
                } else {
                    FlakeRemote::get_url(&flake)
                }
            });
            Some((name, node.locked, handle))
        })
        .collect::<Vec<_>>();

    let updates = handles
        .into_iter()
        .filter_map(|(name, locked, handle)| {
            let remote = handle
                .join()
                .unwrap()
                .map_err(|e| {
                    if !ARGS.quiet {
                        errors::print_error(e);
                    }
                })
                .ok()?;
            (locked?.hash != remote.hash).then_some(name)
        })
        .collect();

    Ok(updates)
}
