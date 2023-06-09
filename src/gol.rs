use std::{cell::RefCell, rc::Rc, vec, time::{Instant, Duration}};

use crate::math::*;

const CHUNK_SIZE: i64 = 13;
struct Cells([bool; (CHUNK_SIZE * CHUNK_SIZE) as usize]);
impl Cells {
    fn get(&self, x: usize, y: usize) -> bool {
        self.0[x + (y * CHUNK_SIZE as usize)]
    }
    fn set(&mut self, x: usize, y: usize, val: bool) {
        self.0[x + (y * CHUNK_SIZE as usize)] = val;
    }
    fn new() -> Self{
        let mut cells = [false; (CHUNK_SIZE*CHUNK_SIZE)as usize];
        for x in 0..CHUNK_SIZE{
            for y in 0..CHUNK_SIZE{
                if x == 0 || y == 0 || x == CHUNK_SIZE-1 || y == CHUNK_SIZE-1{
                    cells[(x + y*CHUNK_SIZE) as usize] = true;
                }
            }
        }
        Cells(cells)
    }
}
pub struct CellChunk {
    border: RefCell<[Option<Rc<CellChunk>>; 8]>,
    cells: RefCell<Cells>,
}
impl CellChunk {
    pub fn new() -> Self {
        CellChunk {
            border: RefCell::new(CellChunk::empty_chunks()),
            cells: RefCell::new(Cells::new()),
        }
    }
    fn empty_chunks() -> [Option<Rc<CellChunk>>; 8] {
        [
            Option::None,
            Option::None,
            Option::None,
            Option::None,
            Option::None,
            Option::None,
            Option::None,
            Option::None,
        ]
    }
    pub fn chunk_to(&self, x: i64, y: i64) -> Rc<CellChunk> {
        self.border.borrow()[dir2index(x, y)]
            .as_ref()
            .unwrap()
            .clone()
    }
}
pub struct World {
    size: Vec4<i64>,
    root: Rc<CellChunk>,
}
impl World {
    pub fn new() -> Self {
        World {
            size: Vec4 {
                x1: 0,
                y1: 0,
                x2: CHUNK_SIZE,
                y2: CHUNK_SIZE,
            },
            root: Rc::new(CellChunk::new()),
        }
    }

    pub fn get_world(&self, win: Vec4<i64>) -> (Vec<bool>, Duration) {
        // Time
        let time = Instant::now();
        // Data
        let win_size = win.size();
        let mut data = vec![false; (win_size.x * win_size.y) as usize];
        // Calculate where window intersects with living world
        let contact = win.intersect(&self.size);
        if contact.is_none() {
            return (data, time.elapsed());
        }
        let contact = contact.unwrap();
        // Calculate needed chunks
        let chunk_x_start = if contact.x1 != 0 {
            (contact.x1 as f64 / CHUNK_SIZE as f64).floor() as i64
        } else {
            0
        };
        let chunk_x_end = if contact.x2 != 0 {
            ((contact.x2 as f64 - 0.001) / CHUNK_SIZE as f64) as i64
        } else {
            0
        };
        let chunk_y_start = if contact.y1 != 0 {
            (contact.y1 as f64 / CHUNK_SIZE as f64).floor() as i64
        } else {
            0
        };
        let chunk_y_end = if contact.y2 != 0 {
            ((contact.y2 as f64 - 0.001) / CHUNK_SIZE as f64) as i64
        } else {
            0
        };
        // Iterate over chunks
        for chunk_y in chunk_y_start..=chunk_y_end {
            for chunk_x in chunk_x_start..=chunk_x_end {
                let chunk_rect = Vec4 {
                    x1: chunk_x * CHUNK_SIZE,
                    x2: chunk_x * CHUNK_SIZE + CHUNK_SIZE,
                    y1: chunk_y * CHUNK_SIZE,
                    y2: chunk_y * CHUNK_SIZE + CHUNK_SIZE,
                };
                // Which cells are needed
                let needed = contact.intersect(&chunk_rect);
                if needed.is_none() {
                    continue;
                }
                let needed = needed.unwrap();
                // Get chunk cells
                let chunk = self.get_chunk(chunk_x, chunk_y);
                let cells = chunk.cells.borrow();
                // Iterate over needed cells and inject them to buffer
                for x in needed.x1..needed.x2 {
                    for y in needed.y1..needed.y2 {
                        let cell_x = (x - chunk_rect.x1) as usize;
                        let cell_y = (y - chunk_rect.y1) as usize;
                        data[((x - win.x1) + ((y - win.y1) * win_size.x)) as usize] =
                            cells.get(cell_x, cell_y);
                    }
                }
            }
        }
        // Time
        (data, time.elapsed())
    }
    fn get_chunk(&self, x: i64, y: i64) -> Rc<CellChunk> {
        if x == 0 && y == 0 {
            return self.root.clone();
        };
        // Determine how much skew and directional moves
        let moves_skew: i64;
        let moves_hor: i64;
        let moves_ver: i64;
        let x_abs = x.abs();
        let y_abs = y.abs();
        if x_abs > y_abs {
            moves_skew = y_abs;
            moves_hor = x_abs - y_abs;
            moves_ver = 0;
        } else if x_abs < y_abs {
            moves_skew = x_abs;
            moves_hor = 0;
            moves_ver = y_abs - x_abs;
        } else {
            moves_skew = x_abs;
            moves_ver = 0;
            moves_hor = 0;
        }
        // Directons to index
        let x_norm = x.signum();
        let y_norm = y.signum();
        let skew_dir = dir2index(x_norm, y_norm);
        let hor_dir = dir2index(x_norm, 0);
        let ver_dir = dir2index(0, y_norm);
        // Travel
        let mut chunk = self.root.clone();
        let mut last_chunk = chunk.clone();
        let mut last_move = skew_dir;
        // Skew
        for _ in 0..moves_skew {
            last_chunk = chunk.clone();
            last_move = skew_dir;
            chunk = chunk.chunk_to(x_norm, y_norm);
        }
        // Horizontal
        for _ in 0..moves_hor {
            last_move = hor_dir;
            last_chunk = chunk.clone();
            chunk = chunk.chunk_to(x_norm, 0);
        }
        // Vertical
        for _ in 0..moves_ver {
            last_move = ver_dir;
            last_chunk = chunk.clone();
            chunk = chunk.chunk_to(0, y_norm);
        }
        // Point from last chunk
        let chunk = last_chunk.border.borrow()[last_move]
            .as_ref()
            .unwrap()
            .clone();
        chunk
    }
    pub fn resize(&mut self, x: i64, y: i64) {
        // ---------X-----------
        if x != 0 {
            for _ in 0..x.abs(){
                // Get first chunk
                let chunk_x = if x.signum() == 1 {
                    (self.size.x2 / CHUNK_SIZE) - 1
                } else {
                    self.size.x1 / CHUNK_SIZE
                };
                let chunk_y = self.size.y1 / CHUNK_SIZE;
                let chunk = self.get_chunk(chunk_x, chunk_y);

                // Create first new chunk
                let mut new_last = Rc::new(CellChunk::new());
                let mut border_last = chunk;
                self.connect(&border_last, &new_last, x.signum(), 0);
                // Iterate downards if necessary 
                let chunk_count = (self.size.y2 - self.size.y1) / CHUNK_SIZE;
                for _ in 0..chunk_count-1{
                    let new = Rc::new(CellChunk::new());
                    let border = border_last.chunk_to(0, 1);

                    self.connect(&new, &new_last, 0, -1);
                    self.connect(&new, &border_last, -x.signum(), -1);
                    self.connect(&new, &border, -x.signum(), 0);
                    self.connect(&border, &new_last, x.signum(), -1);

                    new_last = new;
                    border_last = border;
                }

                // Increase world size
                if x.signum() == 1 {
                    self.size.x2 += CHUNK_SIZE;
                } else {
                    self.size.x1 -= CHUNK_SIZE;
                }
            }
        }
        // ---------Y-----------
        if y != 0 {
            for _ in 0..y.abs(){
                // Get first chunk
                let chunk_y = if y.signum() == 1 {
                    (self.size.y2 / CHUNK_SIZE) - 1
                } else {
                    self.size.y1 / CHUNK_SIZE
                };
                let chunk_x = self.size.x1 / CHUNK_SIZE;
                let chunk = self.get_chunk(chunk_x, chunk_y);

                // Create first new chunk
                let mut new_last = Rc::new(CellChunk::new());
                let mut border_last = chunk;
                self.connect(&border_last, &new_last, 0, y.signum());
                // Iterate right if necessary 
                let chunk_count = (self.size.x2 - self.size.x1) / CHUNK_SIZE;
                for _ in 0..chunk_count-1{
                    let new = Rc::new(CellChunk::new());
                    let border = border_last.chunk_to(1, 0);

                    self.connect(&new, &new_last, -1, 0);
                    self.connect(&new, &border_last, -1, -y.signum());
                    self.connect(&new, &border, 0, -y.signum());
                    self.connect(&border, &new_last, -1, y.signum());

                    new_last = new;
                    border_last = border;
                }

                // Increase world size
                if y.signum() == 1 {
                    self.size.y2 += CHUNK_SIZE;
                } else {
                    self.size.y1 -= CHUNK_SIZE;
                }
            }
        }
    }
    fn connect(&self, chunk1: &Rc<CellChunk>, chunk2: &Rc<CellChunk>, x: i64, y: i64){
        chunk1.border.borrow_mut()[dir2index(x, y)] = Some(chunk2.clone());
        chunk2.border.borrow_mut()[dir2index(-x, -y)] = Some(chunk1.clone());
    }
    pub fn size(&self) -> Vec4<i64> {
        self.size
    }

    pub fn set_cell(&mut self, x: i64, y: i64, state: bool){
        // Cell out of world borders
        let mut expand_x = 0;
        let mut expand_y = 0;
        if x > self.size.x2 {
            expand_x = ((x - self.size.x2) as f64 / x as f64).ceil() as i64;
        }
        if x < self.size.x1 {
            expand_x = ((x - self.size.x1) as f64 / x as f64).floor() as i64;
        }
        if y > self.size.y2 {
            expand_y = ((y - self.size.y2) as f64 / y as f64).ceil() as i64;
        }
        if y < self.size.y1 {
            expand_y = ((y - self.size.y1) as f64 / y as f64).floor() as i64;
        }
        if expand_x != 0 || expand_y != 0{
            self.resize(expand_x, expand_y);
        }
        // Locate chunk
        let chunk_x = (x as f64 / CHUNK_SIZE as f64).floor() as i64;
        let chunk_y = (y as f64 / CHUNK_SIZE as f64).floor() as i64;
        let chunk = self.get_chunk(chunk_x, chunk_y);
        chunk.cells.borrow_mut().set(
            (x - (chunk_x * CHUNK_SIZE)) as usize, 
            (y - (chunk_y * CHUNK_SIZE)) as usize, 
            true
        );
    }
    
}
