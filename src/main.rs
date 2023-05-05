use crossterm::event::{read, Event, KeyEvent, KeyCode,};
use crossterm::terminal::size;
use crossterm::{execute,queue,terminal,Result};
use std::thread::__FastLocalKeyInner;
use std::{io,thread,time::Duration};

fn main() -> Result<()> {
    terminal::enable_raw_mode()?;
    let mut stdout = io::stdout();
    let mut ap = app::new()?;

    loop {
        match read()? {
            Event::Key(event) => handle_key(&event,  &mut ap),
            Event::Mouse(event) => println!("{:?}", event),
            Event::Resize(width, height) => {ap.width=width; ap.height=height},
            _ => {}
        }
        if ap.run == false {break;}
    }
    Ok(())
}

fn handle_key(e: &KeyEvent, a: &mut app){
    match e::code{
        KeyCode::Esc => a.run = false,
        _ => {}
    }
}

struct app{
    run: bool,
    width: u16,
    height: u16,
    init_width: u16,
    init_height: u16,
}

impl app{
    pub fn new() -> Result<app>{
        let (w, h) = crossterm::terminal::size()?;
        let a = app{
            run:true,
            width:w,
            height:h,
            init_height:w,
            init_width:h
        };
        Ok(a)
    }
}