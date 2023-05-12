pub mod app {
            use std::{io::{Stdout, stdout, Write}, hash::BuildHasherDefault, fmt::Display};
            use crossterm::{Result, event::KeyboardEnhancementFlags, style::ResetColor};
            use crossterm::event::{self, KeyEvent, KeyCode, Event, read};
            use crossterm::terminal::{self, enable_raw_mode, disable_raw_mode, SetTitle, EnterAlternateScreen, LeaveAlternateScreen};
            use crossterm::{execute, queue};
            use crossterm::style::{Color, SetBackgroundColor, SetForegroundColor, Print};
            use crossterm::cursor::MoveTo;
            use std::sync::Arc;
            use std::sync::Mutex;
            use tokio::spawn;
    pub struct App{
        pub run: bool,
        pub init_width: u16,
        pub init_height: u16,
        pub out: Stdout,
        win_info: Arc<Mutex<WindowInfo>>,
        evs: Arc<Mutex<Vec<Event>>>,
        mask: Arc<Mutex<CaptureMask>>
    }

    struct CaptureMask{
        pub mouse: bool,
        pub keyboard: bool,
    }
    struct WindowInfo{
        width: u16,
        height: u16
    }
    
    impl App{
        pub fn new() -> Result<App>{
            let (w, h) = crossterm::terminal::size()?;
            let a = App{
                run:true,
                init_width:w,
                init_height:h,
                out: stdout(),
                evs: Arc::new(Mutex::new(Vec::<Event>::new())),
                win_info: Arc::new(Mutex::new(WindowInfo{width: w, height: h})),
                mask: Arc::new(
                    Mutex::new(
                        CaptureMask { mouse: false, keyboard: false }))
            };
            Ok(a)
        }
        pub fn start(&mut self) -> Result<()>{
            enable_raw_mode()?;
            execute!(self.out, EnterAlternateScreen ,SetTitle("GAME OF LIFE"), event::EnableMouseCapture)?;
            spawn(App::read_ev(self.evs.clone(), self.mask.clone()));
            Ok(())
        }

        pub fn exit (&mut self){
            disable_raw_mode().unwrap();
            execute!(self.out, LeaveAlternateScreen, terminal::SetSize(self.init_width, self.init_height)).unwrap();
            self.run = false;
        }
        fn draw_rect(&mut self, x: u16, y: u16, width: u16, height: u16, col: Color ){
            queue!(self.out, MoveTo(x, y), SetBackgroundColor(col)).unwrap();
            for yy in 0..height{
                queue!(self.out, MoveTo(x, y+yy)).unwrap();
                for _ in 0..width{
                    queue!(self.out, Print(" ")).unwrap();
                }
            }
        }

        pub fn show_msg<T: Display>(&mut self, msg: T){
            let msg_str = msg.to_string();
            let (width, height) = self.get_size();
            let r_x = width - (msg_str.len()/2-1) as u16;
            let r_y = height/2 - 2;
            let r_w = (msg_str.len() + 2) as u16;
            let r_h = 3 as u16;
            let col = Color::Cyan;
            self.draw_rect(r_x, r_y, r_w, r_h, col);
            queue!(self.out, Print(msg_str), ResetColor).unwrap();
            self.out.flush().unwrap();
        }

        async fn read_ev(evs: Arc<Mutex<Vec<Event>>>, mask: Arc<Mutex<CaptureMask>>){
            loop{
                let ev = read().unwrap();
                let mask_lock = mask.lock().unwrap();
                let mut evs_lock = evs.lock().unwrap();
                match ev {
                    Event::Key(e)=>{
                        if mask_lock.keyboard{
                            evs_lock.push(ev);
                        }
                    }
                    Event::Mouse(e)=> {
                        if mask_lock.mouse{
                            evs_lock.push(ev);
                        }
                    },
                    _ => {}
                }
            }
        }

        pub fn set_capture(&mut self, key: bool, mouse: bool){
            let mut lock_mask = self.mask.lock().unwrap();
            lock_mask.keyboard = key;
            lock_mask.mouse = mouse;
        }
        
        pub fn next_ev(&self) -> Option<Event>{
            let mut evs_lock = self.evs.lock().unwrap();
            if evs_lock.len() == 0 {return None}
            Some(evs_lock.remove(0))
        }

        fn get_size(&self) -> (u16, u16){
            let lock = self.win_info.lock().unwrap();
            (lock.width, lock.height)
        }
    }
}