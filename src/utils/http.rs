use std::sync::LazyLock;

pub static HTTP_CLIENT: LazyLock<reqwest::blocking::Client> =
    LazyLock::new(reqwest::blocking::Client::new);
