use crate::utils::args::ShellArgs;
use crate::utils::config::CONFIG;
use crate::utils::parse_package_name;
use anyhow::Result;
use std::process::Command;

pub fn shell(args: &ShellArgs) -> Result<()> {
    let packages = args.packages.iter().map(parse_package_name);
    Command::new("nix")
        .arg("shell")
        .args(packages)
        .arg("-c")
        .arg(args.command.as_ref().unwrap_or(&CONFIG.general.shell))
        .spawn()?
        .wait()?;

    Ok(())
}
