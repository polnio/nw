pub mod args;
pub mod config;
pub mod xdg;

pub fn parse_package_name<T: AsRef<str>>(package: T) -> String {
    let package = package.as_ref();
    if package.contains('#') || package.contains(':') {
        package.to_string()
    } else {
        format!("nixpkgs#{}", package)
    }
}
