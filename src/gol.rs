pub mod gol {
    use std::{rc::Rc, cell::{Cell, RefCell}, vec, fmt::write};
    use Box;
    use std::cmp::{min, max};

#[derive(Clone, Copy)]
    pub struct Vec4<T: Sized> {
        pub x1: T,
        pub y1: T,
        pub x2: T,
        pub y2: T,
    }
    impl Vec4<i64>{
        pub fn local_to(&self, local: &Vec4<i64>) -> Vec4<i64>{
            Vec4 {
                x1: local.x1 - self.x1, 
                y1: local.y1 - self.y1, 
                x2: local.x2 - self.x2, 
                y2: local.y2 - self.y2 
            }
        }
    }
    pub struct Vec2<T> {
        pub x: T,
        pub y: T,
    }
    impl <T>Vec2<T>{
        pub fn new(x: T, y: T) -> Vec2<T>{
            Vec2{
                x: x,
                y: y
            }
        }
    }
    // |||||||||   GAME OF LIFE ||||||||||||

    const CHUNK_SIZE: i64 = 8;
    struct Cells([bool; (CHUNK_SIZE*CHUNK_SIZE)as usize]);
    impl Cells{
        fn get(&self, x:usize, y:usize) ->bool{
            self.0[x + (y*CHUNK_SIZE as usize)]
        }
        fn set(&mut self, x:usize, y:usize, val: bool){
            self.0[x + (y*CHUNK_SIZE as usize)] = val;
        }
    }
    pub struct CellChunk {
        border: Rc<Box<RefCell<[Option<CellChunk>; 8]>>>,
        cells: Cells,
    }
    impl CellChunk {
        pub fn new() -> Self {
            CellChunk {
                border: Rc::new(Box::new(RefCell::new(CellChunk::empty_chunks()))),
                cells: Cells([false; CHUNK_SIZE as usize*CHUNK_SIZE as usize]),
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
                size: Vec4 { x1: 0, y1: 0, x2: CHUNK_SIZE, y2: CHUNK_SIZE }, 
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
            // Top left chunk position
            let chunk_top_left = Vec2::new(
                if rect.x1 != 0 {(rect.x1 as f64 / CHUNK_SIZE as f64).floor() as i64} else {0},
                if rect.y1 != 0 {(rect.y1 as f64 / CHUNK_SIZE as f64).floor() as i64} else {0}
            );
            let chunks_x = ((rect.x2 - rect.x1) as f64 / CHUNK_SIZE as f64).ceil() as i64;
            let chunks_y = ((rect.y2 - rect.y1) as f64 / CHUNK_SIZE as f64).ceil() as i64;
            // Data
            let data_size = Vec2::new(
                rect.x2 - rect.x1,
                rect.y2 - rect.y1
            );
            let mut data = vec![false; (data_size.x*data_size.y) as usize];
            let mut write_pos = Vec2::new(0, 0);

            // Chunk x and y
            let mut chunk_tl = self.get_chunk(chunk_top_left.x, chunk_top_left.y);
            let mut chunk = chunk_tl;
            for chunk_y in 0..chunks_y{
                for chunk_x in 0..chunks_x{
                    // Get where window intersects with chunk
                    let chunk_rect = Vec4{
                            x1 : chunk_top_left.x + (chunk_x * CHUNK_SIZE),
                            x2 : chunk_top_left.x + (chunk_x * CHUNK_SIZE) + CHUNK_SIZE,
                            y1 : chunk_top_left.y + (chunk_y * CHUNK_SIZE),
                            y2 : chunk_top_left.y + (chunk_y * CHUNK_SIZE + CHUNK_SIZE)
                        };
                    let data_rect = self.intersect(&chunk_rect, rect).unwrap().local_to(&chunk_rect);

                    // Iterate over data where window intersects in chunk
                    for x in data_rect.x1..=data_rect.x2{
                        for y in data_rect.y1..=data_rect.y2{
                            let data_index = 
                        }
                    }
                }
            }
            vec![true]
        }
        fn get_chunk(&self, x:i64, y:i64) -> & CellChunk{
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
            let mut chunk = &self.root;
            // Skew
            for _ in 0..moves_skew{
                chunk = chunk.border[skew_dir].as_ref().unwrap();
            }
            // Horizontal
            for _ in 0..moves_hor{
                chunk = chunk.border[hor_dir].as_ref().unwrap();
            }
            // Vertical
            for _ in 0..moves_ver{
                chunk = chunk.border[ver_dir].as_ref().unwrap();
            }
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
