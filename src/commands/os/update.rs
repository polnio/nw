use crate::utils::args::OsUpdateArgs;
use crate::utils::nixos;
use crate::utils::no_offline;
use anyhow::Context as _;
use anyhow::Result;

pub fn update(args: &OsUpdateArgs) -> Result<()> {
    no_offline!();

    nixos::update().context("Failed to update nixos")?;

    let mut builder = nixos::Builder::new();
    if args.apply {
        builder.apply();
    }
    if args.bootloader {
        builder.bootloader();
    }
    builder.build()?;

    Ok(())
}
