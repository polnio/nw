use crate::utils::args::FlakeUpdateArgs;
use crate::utils::flake;
use crate::utils::no_offline;
use anyhow::Result;

pub fn list_update(args: &FlakeUpdateArgs) -> Result<()> {
    no_offline();

    let updates = flake::get_updates(args.flake.as_deref())?;

    for name in updates {
        println!("{}", name);
    }

    Ok(())
}
