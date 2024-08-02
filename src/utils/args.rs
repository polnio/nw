use clap::Parser;
use std::sync::LazyLock;

pub static ARGS: LazyLock<Args> = LazyLock::new(Args::parse);

#[derive(clap::Parser)]
pub struct Args {
    #[command(subcommand)]
    pub command: Command,
    #[arg(short, long, global = true)]
    pub quiet: bool,
    #[arg(long, global = true)]
    pub offline: bool,
    #[cfg(feature = "ui")]
    #[arg(long, global = true)]
    pub ui: bool,
}

#[derive(clap::Subcommand)]
pub enum Command {
    Search(SearchArgs),
    Info(InfoArgs),
    Shell(ShellArgs),
    Run(RunArgs),
    Flake(FlakeArgs),
    Os(OsArgs),
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

#[derive(clap::Args)]
pub struct OsArgs {
    #[command(subcommand)]
    pub command: OsCommand,
}

#[derive(clap::Subcommand)]
pub enum OsCommand {
    Build(OsBuildArgs),
    Update(OsUpdateArgs),
    Clean(OsCleanArgs),
}

#[derive(clap::Args, Clone)]
pub struct OsBuildArgs {
    #[arg(short, long)]
    pub update: bool,
    #[arg(short, long)]
    pub apply: bool,
    #[arg(short, long)]
    pub bootloader: bool,
}

#[derive(clap::Args)]
pub struct OsUpdateArgs {
    #[arg(short, long)]
    pub list: bool,
    #[arg(short, long)]
    pub apply: bool,
    #[arg(short, long)]
    pub bootloader: bool,
}

#[derive(clap::Args)]
pub struct OsCleanArgs {
    #[arg(short, long)]
    pub bootloader: bool,
}
