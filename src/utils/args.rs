use clap::Parser;
use std::sync::LazyLock;

pub static ARGS: LazyLock<Args> = LazyLock::new(Args::parse);

#[derive(clap::Parser)]
pub struct Args {
    #[command(subcommand)]
    pub command: Command,
    #[arg(long, global = true)]
    pub offline: bool,
}

#[derive(clap::Subcommand)]
pub enum Command {
    Search(SearchArgs),
    Info(InfoArgs),
    Shell(ShellArgs),
    Run(RunArgs),
    Flake(FlakeArgs),
}

#[derive(clap::Args)]
pub struct SearchArgs {
    pub query: String,
}

#[derive(clap::Args)]
pub struct InfoArgs {
    pub package: String,
}

#[derive(clap::Args)]
pub struct ShellArgs {
    pub packages: Vec<String>,
    #[arg(short, long)]
    pub command: Option<String>,
}

#[derive(clap::Args)]
pub struct RunArgs {
    pub package: String,
    #[arg(last = true, allow_hyphen_values = true)]
    pub args: Vec<String>,
}

#[derive(clap::Args)]
pub struct FlakeArgs {
    #[command(subcommand)]
    pub command: FlakeCommand,
}

#[derive(clap::Subcommand)]
pub enum FlakeCommand {
    Update(FlakeUpdateArgs),
}

#[derive(clap::Args)]
pub struct FlakeUpdateArgs {
    pub flake: Option<String>,
    #[arg(short, long)]
    pub list: bool,
}
