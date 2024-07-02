use crate::utils::args::{FlakeUpdateArgs, ARGS};
use crate::utils::errors;
use crate::utils::flake::metadata::{FlakeMetadata, FlakeMetadataLocksNodesOriginal};
use crate::utils::flake::remote::FlakeRemote;
use anyhow::{Context, Result};

pub fn list_update(args: &FlakeUpdateArgs) -> Result<()> {
    if ARGS.offline {
        eprintln!("Offline mode is not supported");
        std::process::exit(1);
    }

    let flake = args
        .flake
        .as_ref()
        .map_or_else(Default::default, String::as_str);
    let metadata = FlakeMetadata::get(flake).context("Failed to fetch flake metadata")?;

    let handles = metadata.locks.nodes.into_iter().filter_map(|(name, node)| {
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
    });

    let to_update = handles
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
        .collect::<Vec<_>>();

    for name in to_update {
        println!("{}", name);
    }

    Ok(())
}
