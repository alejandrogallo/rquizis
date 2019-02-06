extern crate tui;
extern crate termion;
extern crate yaml_rust;

use std::vec;
use std::io;
use self::termion::event::Key;
use self::termion::raw::IntoRawMode;
use self::tui::Terminal;
use self::tui::backend::TermionBackend;
use self::tui::widgets::{Widget, Block, Borders};
use self::tui::layout::{Layout, Constraint, Direction};

use crate::ui::event::{Event, Events};


pub fn main(words: vec::Vec<yaml_rust::Yaml>) -> Result<(), io::Error> {
    let stdout = io::stdout().into_raw_mode()?;
    let backend = TermionBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    let size = terminal.size()?;
    let events = Events::new();
    terminal.clear();
    loop {
        terminal.draw(|mut f| {
            let chunks = Layout::default()
                            .direction(Direction::Vertical)
                            .margin(1)
                            .constraints([
                                Constraint::Percentage(30),
                                Constraint::Percentage(20),
                                Constraint::Percentage(50)
                            ].as_ref())
                            .split(size);
            Block::default()
                .title("What is this?")
                .borders(Borders::ALL)
                .render(&mut f, chunks[0]);
            Block::default()
                .title("Answer")
                .borders(Borders::ALL)
                .render(&mut f, chunks[2]);
        });
        match events.next()? {
            Event::Input(key) => {
                if key == Key::Char('q') {
                    break
                }
            }
        }
    }
    Ok(())
}
