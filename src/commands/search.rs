use crate::utils::api;
use crate::utils::args::SearchArgs;
use crate::utils::config::CONFIG;
use crate::utils::ext::TabledWidthExt;
use crate::utils::no_offline;
use anyhow::{Context, Result};
use tabled::settings::{peaker::PriorityMax, Style, Width};
use tabled::Table;

pub fn search(args: &SearchArgs) -> Result<()> {
    no_offline();

    let packages = api::get_by_query(args.query.clone()).context("Failed to get packages")?;

    let cols = std::iter::once([
        "Name".to_string(),
        "Version".to_string(),
        "Description".to_string(),
    ]);
    let rows = packages
        .into_iter()
        .map(|p| [p.attr_name, p.pversion, p.description.unwrap_or_default()]);

    let mut table = Table::from_iter(cols.chain(rows));
    if CONFIG.general().ui() {
        table.with(Style::modern_rounded());
    } else {
        table.with(Style::ascii());
    }

    table.with(Width::wrap_terminal().priority(PriorityMax::new(false)));
    println!("{}", table);

    Ok(())
}
