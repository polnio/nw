use tabled::settings::width::{Width, Wrap};
use terminal_size::terminal_size;

pub trait TabledWidthExt {
    fn wrap_terminal() -> Wrap;
}

impl TabledWidthExt for Width {
    fn wrap_terminal() -> Wrap {
        let width = terminal_size().map(|(width, _)| width.0).unwrap_or(100);
        Wrap::new(width as usize)
    }
}
