use anyhow::Error;

pub fn abort(err: Error) -> ! {
    eprintln!("{:?}", err);
    std::process::exit(1);
}
