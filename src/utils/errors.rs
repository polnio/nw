use anyhow::Error;

pub fn print_error(err: Error) {
    eprintln!("{:?}", err);
}

pub fn abort(err: Error) -> ! {
    print_error(err);
    std::process::exit(1);
}
