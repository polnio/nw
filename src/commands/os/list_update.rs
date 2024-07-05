use crate::utils::args::OsUpdateArgs;
use crate::utils::nixos;
use crate::utils::no_offline;
use anyhow::Result;

pub fn list_update(_args: &OsUpdateArgs) -> Result<()> {
    no_offline!();

    let updates = nixos::get_updates()?;

    for name in updates {
        println!("{}", name);
    }

    Ok(())
}
