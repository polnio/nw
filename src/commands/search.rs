use crate::utils::api;
use crate::utils::args::SearchArgs;
use crate::utils::no_offline;
use anyhow::{Context, Result};
use std::io::{stdout, Write};
use tabwriter::TabWriter;

pub fn search(args: &SearchArgs) -> Result<()> {
    no_offline();

    let packages = api::get_by_query(args.query.clone())?;

    let mut tw = TabWriter::new(stdout());
    writeln!(&mut tw, "name\tversion\tdescription").context("Failed to print packages")?;
    for package in packages {
        writeln!(
            &mut tw,
            "{}\t{}\t{}",
            package.attr_name,
            package.pversion,
            package.description.unwrap_or_default()
        )
        .context("Failed to print packages")?;
    }
    tw.flush().context("Failed to print packages")?;

    Ok(())
}
