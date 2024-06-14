use std::sync::LazyLock;

pub static XDG_DIRS: LazyLock<xdg::BaseDirectories> =
    LazyLock::new(|| match xdg::BaseDirectories::with_prefix("nw") {
        Ok(xdg_dirs) => xdg_dirs,
        Err(err) => {
            eprintln!("Failed to generate xdg directories: {}", err);
            std::process::exit(1);
        }
    });
