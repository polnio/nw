use crate::utils::api;
use crate::utils::args::InfoArgs;
use crate::utils::config::CONFIG;
use crate::utils::ext::TabledWidthExt;
use crate::utils::no_offline;
use anyhow::{Context, Result};
use regex::Regex;
use std::sync::LazyLock;
use tabled::settings::peaker::PriorityMax;
use tabled::settings::{Style, Width};
use tabled::Table;

static HTML_TAG_REGEX: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"</?.+>").unwrap());

fn parse_description(description: String) -> String {
    if description.contains("<rendered-html>") {
        HTML_TAG_REGEX
            .replace_all(&description, "")
            .replace("\n", "")
    } else {
        description
    }
}

pub fn info(args: &InfoArgs) -> Result<()> {
    no_offline();

    let response = api::get_by_attr_name(args.package.clone())
        .context("Failed to fetch package informations")?;
    let Some(package) = response else {
        println!("The package {} does not exist", args.package);
        return Ok(());
    };

    let mut table = Table::from_iter([
        ["Name", &package.attr_name],
        ["Version", &package.pversion],
        [
            "Description",
            &package
                .long_description
                .map(parse_description)
                .or(package.description)
                .unwrap_or_default(),
        ],
        ["Homepage", &package.homepage.join(",")],
        ["Declaration", &package.position.unwrap_or_default()],
    ]);

    if CONFIG.general().ui() {
        table.with(Style::rounded().remove_horizontals());
    } else {
        table.with(Style::ascii().remove_horizontal());
    }
    table.with(Width::wrap_terminal().priority(PriorityMax));
    println!("{}", table);

    Ok(())
}
