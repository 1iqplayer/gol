use std::{
    cell::RefCell,
    rc::Rc,
    vec
};

use crate::math::*;

const CHUNK_SIZE: i64 = 8;
struct Cells([bool; (CHUNK_SIZE * CHUNK_SIZE) as usize]);
impl Cells {
    fn get(&self, x: usize, y: usize) -> bool {
        self.0[x + (y * CHUNK_SIZE as usize)]
    }
    fn set(&mut self, x: usize, y: usize, val: bool) {
        self.0[x + (y * CHUNK_SIZE as usize)] = val;
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
            cells: RefCell::new(Cells([true; CHUNK_SIZE as usize * CHUNK_SIZE as usize])),
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
    pub fn chunk_to(&self, x: i64, y:i64)->Rc<CellChunk>{
        self.border.borrow()[dir2index(x, y)].as_ref().unwrap().clone()
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
            root: Rc::new(CellChunk::new())
        }
    }
    
    pub fn get_world(&self, win: Vec4<i64>) -> Vec<bool> {
        // Data
        let win_size = win.size();
        let mut data = vec![false ;(win_size.x * win_size.y) as usize];
        // Calculate where window intersects with living world
        let contact = win.intersect(&self.size);
        if contact.is_none(){return  data;}
        let contact = contact.unwrap();
        // Calculate needed chunks
        let chunk_x_start = if contact.x1 != 0 {(contact.x1 as f64 / CHUNK_SIZE as f64).floor() as i64} else {0};
        let chunk_x_end = if contact.x2 != 0 {((contact.x2 as f64 - 0.001)  / CHUNK_SIZE as f64) as i64} else {0};
        let chunk_y_start = if contact.y1 != 0 {(contact.y1 as f64 / CHUNK_SIZE as f64).floor() as i64} else {0};
        let chunk_y_end = if contact.y2 != 0 {((contact.y2 as f64 - 0.001)  / CHUNK_SIZE as f64) as i64} else {0};
        // Iterate over chunks 
        for chunk_y in chunk_y_start..=chunk_y_end {
            for chunk_x in chunk_x_start..=chunk_x_end{
                let chunk_rect = Vec4{
                    x1 : chunk_x * CHUNK_SIZE,
                    x2 : chunk_x * CHUNK_SIZE + CHUNK_SIZE,
                    y1 : chunk_y * CHUNK_SIZE,
                    y2 : chunk_y * CHUNK_SIZE + CHUNK_SIZE
                };
                // Which cells are needed
                let needed = contact.intersect(&chunk_rect);
                if needed.is_none(){continue;}
                let needed = needed.unwrap();
                // Get chunk cells
                let chunk = self.get_chunk(chunk_x, chunk_y);
                let cells = chunk.cells.borrow();
                // Iterate over needed cells and inject them to buffer
                for x in needed.x1..needed.x2{
                    for y in needed.y1..needed.y2{
                        let cell_x = (x - chunk_rect.x1) as usize;
                        let cell_y = (y - chunk_rect.y1) as usize;
                        data[((x - win.x1) + (((y - win.y1) * win_size.x))) as usize] = cells.get(cell_x, cell_y);
                    }
                }
            }
        }
        data
    }
    fn get_chunk(&self, x: i64, y: i64) -> Rc<CellChunk> {
        if x == 0 && y == 0 {return self.root.clone()};
        // Determine how much skew and directional moves
        let moves_skew: i64;
        let moves_hor: i64;
        let moves_ver: i64;
        if x > y {
            moves_skew = y;
            moves_hor = x - y;
            moves_ver = 0;
        } else if x < y {
            moves_skew = x;
            moves_hor = 0;
            moves_ver = y - x;
        } else {
            moves_skew = x;
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
        let chunk = last_chunk.border.borrow()[last_move].as_ref().unwrap().clone();
        chunk
    }
    pub fn resize(&mut self, x: i64, y:i64){
        // Info
        let wrld_size = self.world_size();
        let chunks_x = wrld_size.0 / CHUNK_SIZE;
        let chunks_y = wrld_size.1 / CHUNK_SIZE;
        let dir_x = x.signum();
        let dir_y = y.signum();
        // Horizontal resize
        if x != 0{
            // Get first chunk
            let mut chunk = if dir_x == 1 {self.get_chunk((self.size.x2 / CHUNK_SIZE)-1, self.size.y1 / CHUNK_SIZE)} else {self.get_chunk(self.size.x1 / CHUNK_SIZE, self.size.y1 / CHUNK_SIZE)};
            // Iterate over chunks add new neighbour
            chunk.border.borrow_mut()[dir2index(dir_x, 0)] = Some(Rc::new(CellChunk::new()));
            for _ in 0..chunks_y-1{
                chunk = chunk.chunk_to(0, 1);
                chunk.border.borrow_mut()[dir2index(dir_x, 0)] = Some(Rc::new(CellChunk::new()));
            }
            // Increase world size
            if dir_x == 1 {
                self.size.x2 += x*CHUNK_SIZE;
            }else{
                self.size.x1 += x*CHUNK_SIZE;
            }
        }
        // Vertical resize
        if y != 0{
            let mut chunk = if dir_y == 1 {self.get_chunk(self.size.x1 / CHUNK_SIZE, (self.size.y2 / CHUNK_SIZE)-1)} else {self.get_chunk(self.size.x1 / CHUNK_SIZE, self.size.y1 / CHUNK_SIZE)};
            chunk.border.borrow_mut()[dir2index(0, dir_y)] = Some(Rc::new(CellChunk::new()));
            for _ in 0..chunks_x-1{
                chunk = chunk.chunk_to(1, 0);
                chunk.border.borrow_mut()[dir2index(0, dir_y)] = Some(Rc::new(CellChunk::new()));
            }
            if dir_y == 1 {
                self.size.y2 += y*CHUNK_SIZE;
            }else{
                self.size.y1 += y*CHUNK_SIZE;
            }
        }
    }
    pub fn world_size(&self) -> (i64, i64){
        (self.size.x2 - self.size.x1, self.size.y2 - self.size.y1)
    }
}