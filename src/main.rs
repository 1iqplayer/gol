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
            Event::Key(k) => handle_key(&mut app, k),
            Event::Mouse(m) => handle_mouse(&mut app, m),
            _ => {}
        }
    }

    Ok(())
}

fn handle_mouse(app: &mut App, m: MouseEvent) {
    match m.kind {
        MouseEventKind::Down(b) => {
            if b == MouseButton::Left {
                app.set_cell(true, m.column, m.row)
            }
        }
        MouseEventKind::Drag(b) => {
            if b == MouseButton::Left {
                app.set_cell(true, m.column, m.row)
            }
        }
        _ => {}
    }
}

fn handle_key(app: &mut App, k: KeyEvent) {
    match k.code {
        KeyCode::Enter => {
            app.show_msg("wiktor szysko", Color::Red, Duration::from_secs(2));
        }
        KeyCode::Esc => app.exit(),
        _ => {}
    }
}
