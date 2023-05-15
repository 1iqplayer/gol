pub mod gol {
    use std::{rc::Rc, ops::{Add, Sub}};
    use Box;
    use std::cmp::{min, max};

#[derive(Clone, Copy)]
    pub struct Vec4<T: Sized> {
        pub x1: T,
        pub y1: T,
        pub x2: T,
        pub y2: T,
    }

    pub struct Vec2<T> {
        pub x: T,
        pub y: T,
    }
    impl Vec2<i64>{
        fn add(&self, x: i64, y: i64) -> Vec2<i64>{
            Vec2 { x: self.x + x, y: self.y + y}
        }
    }


    const CHUNK_SIZE: u8 = 8;
    struct Cells([bool; (CHUNK_SIZE*CHUNK_SIZE)as usize]);
    impl Cells{
        fn get(&self, x:u8, y:u8) ->bool{
            self.0[x as usize + (y*CHUNK_SIZE) as usize]
        }
        fn set(&mut self, x:u8, y:u8, val: bool){
            self.0[x as usize + (y*CHUNK_SIZE) as usize] = val;
        }
    }
    pub struct CellChunk {
        border: Rc<Box<[Option<CellChunk>; 8]>>,
        cells: Cells,
    }
    impl CellChunk {
        pub fn new() -> Self {
            CellChunk {
                border: Rc::new(Box::new(CellChunk::empty_chunks())),
                cells: Cells([false; (CHUNK_SIZE*CHUNK_SIZE)as usize]),
            }
        }
        
        pub fn empty_chunks() -> [Option<CellChunk>; 8]{
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
    }
    pub struct World {
        size: Vec4<i64>,
        root: CellChunk,
    }
    impl World{
        pub fn new()->Self{
            World { 
                size: Vec4 { x1: 0, y1: 0, x2: CHUNK_SIZE as i64, y2: CHUNK_SIZE as i64 }, 
                root: CellChunk::new() 
            }
        }
        fn contains(&self, p: &Vec2<i64>, s: &Vec4<i64>) -> bool{
            p.x > s.x1 && p.x < s.x2 && p.y > s.y1 && p.y < s.y2
        }
        pub fn get_world(&self, win: Vec4<i64>) -> Option<Vec<bool>>{
            // Check what corners of window are in living world
            let wrld = self.size;
            // Calculate data rectangle
            let data_rect = Vec4{
                x1: max(wrld.x1, win.x1),
                x2: min(wrld.x2, win.x2),
                y1: max(wrld.y1, win.y1),
                y2: min(wrld.y2, win.y2)
            };
            // Window not in world
            if data_rect.x1 == data_rect.x2 || data_rect.y1 == data_rect.y2{return None}
            // Get values
            let data = self.get_data(&data_rect);
            None
        }
        fn get_data(&self, rect: &Vec4<i64>)-> Vec<bool>{
            let modulo: Vec4<i64> = Vec4 { 
                x1: rect.x1%CHUNK_SIZE as i64, 
                y1: rect.y1%CHUNK_SIZE as i64, 
                x2: rect.x2%CHUNK_SIZE as i64, 
                y2: rect.y2%CHUNK_SIZE as i64
            };
        }
    }
}
