use crossterm::Result;
use crossterm::cursor::position;
use crossterm::event::{Event, KeyCode, KeyEvent};
use gol::app::App;


#[tokio::main]
async fn main() -> Result<()> {
    let mut app = App::new()?;
    app.start().unwrap();
    app.set_capture(true, false);
    app.show_msg("dzike chuje");

    while app.run{
        let ev = app.next_ev();
        if ev != None{
            match ev.unwrap(){
                Event::Key(key) =>{
                    if key.code == KeyCode::Esc{
                        app.exit();
                    }
                    println!("{:?}", key.code);
                },
                _ => {}
            }
        }
    }

    Ok(())
}