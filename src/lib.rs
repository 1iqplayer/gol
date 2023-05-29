mod gol;
mod math;
use crate::gol::*;
use crate::math::*;
use crossterm::cursor::{Hide, MoveDown, MoveTo, Show};
use crossterm::event;
use crossterm::style::{Color, Print, SetBackgroundColor, SetForegroundColor};
use crossterm::terminal::{
    self, disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen,
    SetTitle,
};
use crossterm::Result;
use crossterm::{execute, queue};
use std::io::Write;
use std::io::{stdout, Stdout};
pub struct App {
    pub run: bool,
    win_info_init: Vec2<u16>,
    win_info: Vec4<i64>,
    pub out: Stdout,
    world: World,
}

impl App {
    pub fn new() -> Result<App> {
        let (w, h) = crossterm::terminal::size()?;

        let a = App {
            run: true,
            win_info_init: Vec2 { x: w, y: h },
            win_info: Vec4 {
                x1: 0,
                y1: 0,
                x2: w as i64,
                y2: h as i64,
            },
            out: stdout(),
            world: World::new(),
        };
        Ok(a)
    }
    pub fn start(&mut self) -> Result<()> {
        enable_raw_mode()?;
        execute!(
            self.out,
            EnterAlternateScreen,
            SetTitle("GAME OF LIFE"),
            event::EnableMouseCapture,
            Hide
        )?;
        Ok(())
    }

    pub fn exit(&mut self) {
        disable_raw_mode().unwrap();
        execute!(
            self.out,
            LeaveAlternateScreen,
            terminal::SetSize(self.win_info_init.x, self.win_info_init.y),
            Show
        )
        .unwrap();
        self.run = false;
    }
    fn draw_rect(&mut self, x: u16, y: u16, width: u16, height: u16, col: Color) {
        queue!(self.out, MoveTo(x, y), SetBackgroundColor(col)).unwrap();
        for yy in 0..height {
            queue!(self.out, MoveTo(x, y + yy)).unwrap();
            for _ in 0..width {
                queue!(self.out, Print(" ")).unwrap();
            }
        }
    }

    pub fn draw(&mut self) {
        // Draw cells
        let data = self.world.get_world(self.win_info);
        let win_size = self.win_info.size();
        for y in 0..win_size.y {
            queue!(self.out, MoveTo(0, y as u16)).unwrap();
            for x in 0..win_size.x {
                let cell = data[(x + (y * win_size.x)) as usize];
                if cell {
                    queue!(self.out, SetBackgroundColor(Color::Cyan)).unwrap();
                } else {
                    queue!(self.out, SetBackgroundColor(Color::Black)).unwrap();
                }
                queue!(self.out, Print(" ")).unwrap();
            }
        }
        // Draw info
        let world_size = self.world.size_raw();
        let win_str = format!(
            "WINDOW  X:{} Y:{} W:{} H:{}",
            self.win_info.x1, self.win_info.y1, win_size.x, win_size.y
        );
        let wrld_str = format!(
            "WORLD  X1:{} Y1:{} X2:{} Y2:{}",
            world_size.x1, world_size.y1, world_size.x2, world_size.y2
        );
        queue!(
            self.out,
            SetBackgroundColor(Color::White),
            SetForegroundColor(Color::Black),
            MoveTo(
                (win_size.x - wrld_str.len() as i64) as u16,
                (win_size.y - 2) as u16
            ),
            Print(wrld_str),
            MoveTo(
                (win_size.x - win_str.len() as i64) as u16,
                (win_size.y - 1) as u16
            ),
            Print(win_str)
        )
        .unwrap();
        self.out.flush().unwrap();
    }
    pub fn move_window(&mut self, x: i64, y: i64) {
        self.win_info.x1 += x;
        self.win_info.x2 += x;
        self.win_info.y1 += y;
        self.win_info.y2 += y;
        self.draw();
    }
    pub fn resize(&mut self, x: i64, y: i64) {
        self.world.resize(x, y);
        self.draw();
    }

    // pub fn show_msg<T: Display>(&mut self, msg: T, col: Color, duration: Duration) {
    //     let msg_len = msg.to_string().len() as u16;
    //     let (width, height) = (self.win_info.x, self.win_info.y);
    //     let r_x = width / 2 - (msg_len / 2) - 1 as u16;
    //     let r_y = height / 2 - 1;
    //     let r_w = (msg_len + 2) as u16;
    //     let r_h = 3 as u16;
    //     self.draw_rect(r_x, r_y, r_w, r_h, col);
    //     queue!(
    //         self.out,
    //         MoveTo(width / 2 - msg_len / 2, height / 2),
    //         Print(msg),
    //         ResetColor
    //     )
    //     .unwrap();
    //     self.out.flush().unwrap();
    //     sleep(duration);
    //     queue!(self.out, ResetColor).unwrap();
    // }
}