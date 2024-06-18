use crate::utils::errors::abort;
use anyhow::Context;
use std::sync::LazyLock;

pub static XDG_DIRS: LazyLock<xdg::BaseDirectories> = LazyLock::new(|| {
    match xdg::BaseDirectories::with_prefix("nw").context("Failed to generate xdg directories") {
        Ok(xdg_dirs) => xdg_dirs,
        Err(err) => abort(err),
    }
});
