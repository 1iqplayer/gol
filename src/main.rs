use crossterm::event::{Event, KeyCode};
use crossterm::terminal::SetSize;
use crossterm::{Result, execute};
use gol::App;

fn main() -> Result<()> {
    let mut app = App::new()?;
    app.start().unwrap();

    while app.run {
        match crossterm::event::read().unwrap() {
            Event::Key(k) => handle_key(&mut app, k.code),
            Event::Mouse(m) => {}
            _ => {}
        }
    }

    Ok(())
}
fn handle_key(app: &mut App, k: KeyCode) {
    match k {
        KeyCode::Enter => {
            app.draw();
        }
        KeyCode::Esc => {
            app.exit();
        }
        KeyCode::Left => {
            app.move_window(-2, 0);
        }
        KeyCode::Right => {
            app.move_window(2, 0);
        }
        KeyCode::Up => {
            app.move_window(0, -2);
        }
        KeyCode::Down => {
            app.move_window(0, 2);
        }
        KeyCode::Char(c) => {
            match c {
                'd' => {
                    app.resize(1, 0);
                }
                'a' => {
                    app.resize(-1, 0);
                }
                's' => {
                    app.resize(0, 1);
                }
                'w' => {
                    app.resize(0, -1);
                }
                _ => {}
            }
            {}
        }
        _ => {}
    }
}
