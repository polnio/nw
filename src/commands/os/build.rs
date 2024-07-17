use crate::utils::args::OsBuildArgs;
use crate::utils::nixos;
use crate::utils::no_offline;
use anyhow::Result;

pub fn build(args: &OsBuildArgs) -> Result<()> {
    if args.update {
        no_offline();
        nixos::update()?;
    }
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
