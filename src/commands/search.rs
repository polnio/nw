use crate::utils::api;
use crate::utils::args::SearchArgs;
use crate::utils::args::ARGS;
use anyhow::{Context, Result};
use std::io::{stdout, Write};
use tabwriter::TabWriter;

pub fn search(args: &SearchArgs) -> Result<()> {
    if ARGS.offline {
        eprintln!("Offline mode is not supported");
        std::process::exit(1);
    }

    let packages = api::get_by_query(args.query.clone())?;

    let mut tw = TabWriter::new(stdout());
    write!(&mut tw, "name\tversion\tdescription\n").context("Failed to print packages")?;
    for package in packages {
        write!(
            &mut tw,
            "{}\t{}\t{}\n",
            package.attr_name,
            package.pversion,
            package.description.unwrap_or_default()
        )
        .context("Failed to print packages")?;
    }
    tw.flush().context("Failed to print packages")?;

    Ok(())
}
