use crate::utils::{api, args::SearchArgs};
use anyhow::{Context, Result};
use std::io::{stdout, Write};
use tabwriter::TabWriter;

pub fn search(args: &SearchArgs) -> Result<()> {
    let packages = api::get_by_query(args.query.clone())?;

    let mut tw = TabWriter::new(stdout());
    write!(&mut tw, "name\tversion\tdescription\n").context("Failed to print packages")?;
    for package in packages {
        write!(
            &mut tw,
            "{}\t{}\t{}\n",
            package.pname,
            package.pversion,
            package.description.unwrap_or_default()
        )
        .context("Failed to print packages")?;
    }
    tw.flush().context("Failed to print packages")?;

    Ok(())
}
