pub mod gol {
    use std::{rc::Rc, cell::Cell};
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


    const CHUNK_SIZE: usize = 8;
    struct Cells([bool; (CHUNK_SIZE*CHUNK_SIZE)as usize]);
    impl Cells{
        fn get(&self, x:usize, y:usize) ->bool{
            self.0[x + (y*CHUNK_SIZE)]
        }
        fn set(&mut self, x:usize, y:usize, val: bool){
            self.0[x + (y*CHUNK_SIZE)] = val;
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
                cells: Cells([false; CHUNK_SIZE*CHUNK_SIZE]),
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
        fn intersect(&self, r1: &Vec4<i64>, r2: &Vec4<i64>) -> Option<Vec4<i64>>{
            let rect = Vec4 { 
                x1: max(r1.x1, r2.x1),
                y1: max(r1.y1, r2.y1),
                x2: min(r1.x2, r2.x2),
                y2: min(r1.y2, r2.y2)
            };
            if rect.x1 == rect.x2 || rect.y1 == rect.y2{return None}
            Some(rect)
        }
        pub fn get_world(&self, win: Vec4<i64>) -> Option<Vec<bool>>{
            let wrld = self.size;
            // Calculate where window intersects with living world
            let data_rect = self.intersect(&win, &wrld);
            // Window not in world
            if data_rect.is_none(){return  None}
            // Get values
            let data = self.get_data(&data_rect.unwrap());
            None
        }
        fn get_data(&self, rect: &Vec4<i64>)-> Vec<bool>{
            // Which chunks contains data
            let chunk_x_start = if rect.x1 != 0 {(rect.x1 as f64 / CHUNK_SIZE as f64).floor() as i64} else {0};
            let chunk_x_end =  (rect.x2 as f64 / CHUNK_SIZE as f64).ceil() as i64;
            let chunk_y_start = if rect.y1 != 0 {(rect.y1 as f64 / CHUNK_SIZE as f64).floor() as i64} else {0};
            let chunk_y_end =  (rect.y2 as f64 / CHUNK_SIZE as f64).ceil() as i64;
            // Data
            let mut data = Vec::<bool>::new();

            for chunk_y in chunk_y_start..=chunk_y_end{
                for chunk_x in chunk_x_start..=chunk_x_end{
                    let data_rect = self.intersect(
                        &Vec4{
                            x1 : chunk_x * CHUNK_SIZE as i64,
                            x2 : chunk_x * CHUNK_SIZE as i64 + CHUNK_SIZE as i64,
                            y1 : chunk_y * CHUNK_SIZE as i64,
                            y2 : chunk_y * CHUNK_SIZE as i64 + CHUNK_SIZE as i64
                        }, 
                        rect
                    ).unwrap();
                    // Iterate over data where window intersects in chunk
                    let chunk = self.get_chunk(chunk_x, chunk_y);

                    for x in data_rect.x1..data_rect.x2{
                        for y in data_rect.y1..data_rect.y2{

                        }
                    }
                }
            }
            vec![true]
        }
        fn get_chunk(&mut self, x:i64, y:i64) -> &mut CellChunk{
            // Determine how much skew and directional moves
            let moves_skew: i64;
            let moves_hor: i64;
            let moves_ver: i64;
            if x>y{
                moves_skew = y;
                moves_hor = x-y;
                moves_ver = 0;
            }else if x<y{
                moves_skew = x;
                moves_hor = 0;
                moves_ver = y-x;
            }else{
                moves_skew = y;
                moves_hor = 0;
                moves_ver = 0;
            }
            // Directons to index
            let x_norm = x.signum();
            let y_norm = y.signum();
            let skew_dir = self.dir2index(x_norm, y_norm);
            let hor_dir = self.dir2index(x_norm, 0);
            let ver_dir = self.dir2index(0, y_norm);
            // Travel
            let mut chunk: &mut CellChunk = &mut self.root;
            // Skew
            for _ in 0..moves_skew{
                chunk = chunk.border[skew_dir].as_mut().unwrap();
            }
            // Horizontal
            for _ in 0..moves_hor{
                chunk = &mut chunk.border[hor_dir].as_ref().unwrap();
            }
            // Vertical
            for _ in 0..moves_ver{
                chunk = &mut chunk.border[ver_dir].as_ref().unwrap();
            }
            chunk.cells.set(0, 0, false);
            chunk
        }
        fn dir2index(&self, x: i64, y: i64) -> usize{
            match (x, y){
                (1, 0)=>0,
                (1, 1)=>1,
                (0, 1)=>2,
                (-1, 1)=>3,
                (-1, 0)=>4,
                (-1, -1)=>5,
                (0, -1)=>6,
                (1, -1)=>7,
                _ => 0
            }
            }
        }
    }
