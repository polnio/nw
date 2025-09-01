use std::sync::LazyLock;

pub static XDG_DIRS: LazyLock<xdg::BaseDirectories> =
    LazyLock::new(|| xdg::BaseDirectories::with_prefix("nw"));
