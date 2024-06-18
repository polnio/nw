use crate::utils::args::RunArgs;
use crate::utils::parse_package_name;
use anyhow::Result;
use std::process::Command;

pub fn run(args: &RunArgs) -> Result<()> {
    let package = parse_package_name(&args.package);
    Command::new("nix")
        .arg("run")
        .arg(package)
        .arg("--")
        .args(&args.args)
        .spawn()?
        .wait()?;

    Ok(())
}
