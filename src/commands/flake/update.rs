use crate::utils::args::FlakeUpdateArgs;
use crate::utils::flake;
use crate::utils::no_offline;
use anyhow::Context as _;
use anyhow::Result;

pub fn update(args: &FlakeUpdateArgs) -> Result<()> {
    no_offline!();

    flake::update(args.flake.as_deref()).context("Failed to update flake")?;

    Ok(())
}
