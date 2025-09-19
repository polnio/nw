use clap::Parser;
use std::num::NonZeroUsize;
use std::sync::LazyLock;

pub static ARGS: LazyLock<Args> = LazyLock::new(Args::parse);

#[derive(clap::Parser)]
#[command(name = "nw")]
pub struct Args {
    #[command(subcommand)]
    pub command: Command,

    /// Less verbose output
    #[arg(short, long, global = true)]
    pub quiet: bool,

    /// Disable network access
    #[arg(long, global = true)]
    pub offline: bool,

    /// Show fancy UI
    #[cfg(feature = "ui")]
    #[arg(long, global = true)]
    pub ui: bool,

    /// Don't show fancy UI
    #[cfg(feature = "ui")]
    #[arg(long, global = true)]
    pub no_ui: bool,

    /// Maximum number of tasks to run in parallel
    #[arg(long, global = true)]
    pub max_tasks: Option<NonZeroUsize>,

    /// Maximum number of cores to use per task
    #[arg(long, global = true)]
    pub max_cores: Option<NonZeroUsize>,
}

#[derive(clap::Subcommand)]
pub enum Command {
    /// Search in nixpkgs. Show the same result as https://search.nixos.org/packages
    Search(SearchArgs),
    /// Fetch information about a package
    Info(InfoArgs),
    /// Use `nix-locate `under the hood
    Locate(LocateArgs),
    /// Like `nix shell`, but nicer
    Shell(ShellArgs),
    /// Like `nix build`, but nicer
    Build(BuildArgs),
    /// Like `nix run`, but nicer
    Run(RunArgs),
    /// Run `nw flake help` for more information
    Flake(FlakeArgs),
    /// Run `nw os help` for more information
    Os(OsArgs),
}

#[derive(clap::Args)]
pub struct SearchArgs {
    /// The search query
    pub query: String,
}

#[derive(clap::Args)]
pub struct InfoArgs {
    /// The package to show information about
    pub package: String,
}

#[derive(clap::Args)]
pub struct LocateArgs {
    /// The search query
    pub query: String,
}

#[derive(clap::Args)]
pub struct ShellArgs {
    /// The packages that will be added to the shell
    pub packages: Vec<String>,
    #[arg(short, long)]
    /// Optionnal command to run in the shell.
    pub command: Option<String>,
    #[arg(short, long)]
    /// Run `nix develop` instead of `nix shell`
    pub dev: bool,
}

#[derive(clap::Args)]
pub struct BuildArgs {
    /// The package to build
    pub package: String,
}

#[derive(clap::Args)]
pub struct RunArgs {
    /// The package to run
    pub package: String,
    #[arg(last = true, allow_hyphen_values = true)]
    /// Extra arguments to pass to the program
    pub args: Vec<String>,
}

#[derive(clap::Args)]
pub struct FlakeArgs {
    #[command(subcommand)]
    pub command: FlakeCommand,
}

#[derive(clap::Subcommand)]
pub enum FlakeCommand {
    /// Update a flake
    Update(FlakeUpdateArgs),
}

#[derive(clap::Args)]
pub struct FlakeUpdateArgs {
    // The flake to update.
    pub flake: Option<String>,
    // List the updates without applying them.
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
    /// Like `nixos-rebuild`, but nicer
    Build(OsBuildArgs),
    /// Update the os flake, and rebuild if necessary
    Update(OsUpdateArgs),
    /// Like `nix-collect-garbage`, but nicer
    Clean(OsCleanArgs),
}

#[derive(clap::Args, Clone)]
pub struct OsBuildArgs {
    /// Update the flake before building
    #[arg(short, long)]
    pub update: bool,
    /// Apply the changes to the system (like `nixos-rebuild test`)
    #[arg(short, long)]
    pub apply: bool,
    /// Add entry to the bootloader (like `nixos-rebuild boot`)
    #[arg(short, long)]
    pub bootloader: bool,
}

#[derive(clap::Args)]
pub struct OsUpdateArgs {
    /// List the updates without applying them
    #[arg(short, long)]
    pub list: bool,
    /// Add entry to the bootloader (like `nixos-rebuild test`)
    #[arg(short, long)]
    pub apply: bool,
    /// Add entry to the bootloader (like `nixos-rebuild boot`)
    #[arg(short, long)]
    pub bootloader: bool,
}

#[derive(clap::Args)]
pub struct OsCleanArgs {
    /// remove entries of the bootloader (like `nix-collect-garbage -d`)
    #[arg(short, long)]
    pub bootloader: bool,
}
