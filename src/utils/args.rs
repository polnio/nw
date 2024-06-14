use clap::Parser;
use std::sync::LazyLock;

pub static ARGS: LazyLock<Args> = LazyLock::new(Args::parse);

#[derive(clap::Parser)]
pub struct Args {
    #[command(subcommand)]
    pub command: Command,
}

#[derive(clap::Subcommand)]
pub enum Command {
    Shell(ShellArgs),
    Run(RunArgs),
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
