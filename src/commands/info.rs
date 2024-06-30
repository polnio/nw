use crate::utils::{api, args::InfoArgs};
use anyhow::{Context, Result};
use regex::Regex;
use std::io::{stdout, Write};
use std::sync::LazyLock;
use tabwriter::TabWriter;

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
    let response = api::get_by_attr_name(args.package.clone())
        .context("Failed to fetch package informations")?;
    let Some(package) = response else {
        println!("The package {} does not exist", args.package);
        return Ok(());
    };
    let mut tw = TabWriter::new(stdout());
    write!(
        tw,
        "Name\t: {}\nVersion\t: {}\nDescription\t: {}\nHomepage\t: {}\nDeclaration\t: {}\n",
        package.attr_name,
        package.pversion,
        package
            .long_description
            .map(parse_description)
            .or(package.description)
            .unwrap_or(console::style("No description found").italic().to_string()),
        package.homepage.join(","),
        package
            .position
            .unwrap_or(console::style("No declaration found").italic().to_string())
    )
    .context("Failed to print package informations")?;
    tw.flush().context("Failed to print package informations")?;
    Ok(())
}
