pub mod api;
pub mod args;
pub mod config;
pub mod errors;
pub mod flake;
pub mod http;
pub mod nixos;
pub mod xdg;

pub fn parse_package_name<T: AsRef<str>>(package: T) -> String {
    let package = package.as_ref();
    if package.contains('#') || package.contains(':') {
        package.into()
    } else {
        format!("nixpkgs#{}", package)
    }
}

macro_rules! no_offline {
    () => {
        if crate::args::ARGS.offline {
            eprintln!("Offline mode is not supported");
            std::process::exit(1);
        }
    };
}

pub(crate) use no_offline;
