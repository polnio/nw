#![feature(try_blocks)]

mod commands;
mod utils;

use anyhow::Context;
use utils::args::{self, ARGS};
use utils::errors::abort;

fn main() {
    let result = match &ARGS.command {
        args::Command::Run(args) => commands::run(args).context("Failed to run package"),
        args::Command::Shell(args) => commands::shell(args).context("Failed to start shell"),
        args::Command::Search(args) => commands::search(args).context("Failed to search package"),
        args::Command::Info(args) => commands::info(args).context("Failed to fetch package info"),
    };

    if let Err(err) = result {
        abort(err)
    }
}
