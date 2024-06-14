mod commands;
mod utils;

use utils::args::{self, ARGS};

fn main() {
    let result = match &ARGS.command {
        args::Command::Run(args) => commands::run(args),
        args::Command::Shell(args) => commands::shell(args),
    };

    if let Err(err) = result {
        eprintln!("An error occuried: {}", err);
        std::process::exit(1);
    }
}
