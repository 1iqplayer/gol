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
            Event::Key(k) => {handle_key(&mut app, k.code)},
            Event::Mouse(m)=>{},
            _ => {}
        }
    }

    Ok(())
}
fn handle_key(app: &mut App, k: KeyCode){
    match k{
        KeyCode::Enter => {app.draw();}
        _ => {}
    }
}