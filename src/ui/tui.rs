extern crate tui;
extern crate termion;

use std::io;
use self::tui::Terminal;
use self::tui::backend::TermionBackend;
use self::tui::widgets::{Widget, Block, Borders};
use self::termion::raw::IntoRawMode;

pub fn main() -> Result<(), io::Error> {
    let stdout = io::stdout().into_raw_mode()?;
    let backend = TermionBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    let size = terminal.size()?;
    terminal.clear();
    terminal.draw(|mut f| {
        Block::default()
            .title("Block")
            .borders(Borders::ALL)
            .render(&mut f, size);
    })
}
