use crossterm::event::{Event, KeyCode, KeyEvent, MouseButton, MouseEvent, MouseEventKind};
use crossterm::style::Color;
use crossterm::Result;
use gol::app::App;
use std::time::Duration;

fn main() -> Result<()> {
    let mut app = App::new()?;
    app.start().unwrap();

    while app.run {
        match crossterm::event::read().unwrap() {
            Event::Key(k) => {},
            Event::Mouse(m)=>{},
            _ => {}
        }
    }

    Ok(())
}
