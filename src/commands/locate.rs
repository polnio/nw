use crate::utils::args::LocateArgs;
use crate::utils::no_offline;
use anyhow::{Context as _, Result};
use subprocess::Exec;

pub fn locate(args: &LocateArgs) -> Result<()> {
    no_offline();

    let extension = args.query.rsplit_once('.').map(|(_, ext)| ext);
    let (at_root, path) = extension
        .map(|extension| match extension {
            "so" | "a" => (true, "/lib"),
            "pc" => (true, "/lib/pkgconfig"),
            "h" | "hpp" => (true, "/include"),
            _ => Default::default(),
        })
        .unwrap_or_default();

    Exec::shell(format!(
        "nix-locate --whole-name {} {}/{} | cut -d ' ' -f 1",
        at_root.then_some("--at-root").unwrap_or_default(),
        path,
        args.query
    ))
    .join()
    .context("Failed to run nix-locate")?;

    Ok(())
}
