pub mod api;
pub mod args;
pub mod config;
pub mod errors;
pub mod flake;
pub mod http;
pub mod nixos;
pub mod xdg;

use config::CONFIG;

pub fn parse_package_name<T: AsRef<str>>(package: T) -> String {
    let package = package.as_ref();
    if package.contains('#') || package.contains(':') {
        package.into()
    } else {
        format!("nixpkgs/{}#{}", CONFIG.nix().channel(), package)
    }
}

pub fn no_offline() {
    if crate::args::ARGS.offline {
        eprintln!("Offline mode is not supported");
        std::process::exit(1);
    }
}
