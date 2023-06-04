use crossterm::event::{Event, KeyCode, MouseEvent, MouseEventKind, MouseButton};
use crossterm::terminal::SetSize;
use crossterm::{Result, execute};
use gol::App;

fn main() -> Result<()> {
    let mut app = App::new()?;
    app.start().unwrap();

    while app.run {
        match crossterm::event::read().unwrap() {
            Event::Key(k) => app.handle_key(k.code),
            Event::Mouse(m) => app.handle_mouse(m),
            Event::Resize(w, h) => app.handle_resize(w, h),
            _ => {}
        }
    }

    Ok(())
}
