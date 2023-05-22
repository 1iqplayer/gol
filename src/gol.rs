pub mod gol {
    use std::borrow::{BorrowMut, Borrow};
    use std::cmp::{max, min};
    use std::ops::Deref;
    use std::{
        cell::{Cell, RefCell},
        rc::Rc,
        vec,
    };
    use Box;

    #[derive(Clone, Copy)]
    pub struct Vec4<T: Sized> {
        pub x1: T,
        pub y1: T,
        pub x2: T,
        pub y2: T,
    }
    impl Vec4<i64> {
        pub fn local_to(&self, root: &Vec4<i64>) -> Vec4<i64> {
            Vec4 {
                x1: self.x1 - root.x1,
                y1: self.y1 - root.y1,
                x2: (self.x1 - root.x1) + (self.x2 - self.x1),
                y2: (self.y1 - root.y1) + (self.y2 - self.y1),
            }
        }
        pub fn size(&self)->Vec2<i64>{
            Vec2::new(self.x2 - self.x1, self.y2 - self.y1)
        }
    }
    pub struct Vec2<T> {
        pub x: T,
        pub y: T,
    }
    impl<T> Vec2<T> {
        pub fn new(x: T, y: T) -> Vec2<T> {
            Vec2 { x: x, y: y }
        }
    }
    fn dir2index(x: i64, y: i64) -> usize {
        match (x, y) {
            (1, 0) => 0,
            (1, 1) => 1,
            (0, 1) => 2,
            (-1, 1) => 3,
            (-1, 0) => 4,
            (-1, -1) => 5,
            (0, -1) => 6,
            (1, -1) => 7,
            _ => 0,
        }
    }
    // |||||||||   GAME OF LIFE ||||||||||||

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
        border: [Option<Rc<CellChunk>>; 8],
        cells: RefCell<Cells>,
    }
    impl CellChunk {
        pub fn new() -> Self {
            CellChunk {
                border: CellChunk::empty_chunks(),
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
            self.border[dir2index(x, y)].as_ref().unwrap().clone()
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
        fn intersect(&self, r1: &Vec4<i64>, r2: &Vec4<i64>) -> Option<Vec4<i64>> {
            let rect = Vec4 {
                x1: max(r1.x1, r2.x1),
                y1: max(r1.y1, r2.y1),
                x2: min(r1.x2, r2.x2),
                y2: min(r1.y2, r2.y2),
            };
            if rect.x1 == rect.x2 || rect.y1 == rect.y2 {
                return None;
            }
            Some(rect)
        }
        pub fn get_world(&self, win: Vec4<i64>) -> Option<Vec<bool>> {
            let wrld = self.size;
            // Calculate where window intersects with living world
            let data_rect = self.intersect(&win, &wrld);
            // Window not in world
            if data_rect.is_none() {
                return None;
            }
            // Data
            let data_rect = data_rect.unwrap();
            let data_size = data_rect.size();
            let mut data = vec![false; (data_size.x * data_size.y) as usize];
            self.get_data(&data_rect, &mut data);
            // Inject data into window buffer
            let win_size = win.size();
            let mut win_data = vec![false; (win_size.x*win_size.y) as usize];
            for x in data_rect.x1..data_rect.x2{
                for y in data_rect.y1..data_rect.y2{
                    win_data[((x - win.x1) + (y-win.y1)*win_size.y) as usize] = data[((x-data_rect.x1)+((y-data_rect.y1)*data_size.y)) as usize];
                }
            }
            Some(win_data)
        }
        fn get_data(&self, rect: &Vec4<i64>, data: &mut Vec<bool>){
            // chunk positions
            let chunk_x_start = if rect.x1 != 0 {(rect.x1 as f64 / CHUNK_SIZE as f64).floor() as i64} else {0};
            let chunk_x_end = if rect.x2 != 0 {(rect.x2 as f64 / CHUNK_SIZE as f64).ceil() as i64} else {0};
            let chunk_y_start = if rect.y1 != 0 {(rect.y1 as f64 / CHUNK_SIZE as f64).floor() as i64} else {0};
            let chunk_y_end = if rect.y2 != 0 {(rect.y2 as f64 / CHUNK_SIZE as f64).ceil() as i64} else {0};
            let rect_size = rect.size();

            // Top left chunk 
            let mut chunk_tl = self.get_chunk(chunk_x_start, chunk_y_start);
            let mut chunk = chunk_tl.clone();

            // Iterate trought every chunk
            for chunk_y in chunk_y_start..=chunk_y_end {
                for chunk_x in chunk_x_start..=chunk_x_end{
                    let chunk_rect = Vec4{
                        x1 : chunk_x * CHUNK_SIZE,
                        x2 : chunk_x * CHUNK_SIZE + CHUNK_SIZE,
                        y1 : chunk_y * CHUNK_SIZE,
                        y2 : chunk_y * CHUNK_SIZE + CHUNK_SIZE
                    };
                    
                    let chunk_inter = self.intersect(rect, &chunk_rect);
                    if chunk_inter.is_none(){continue;}
                    let needed = chunk_inter.unwrap();
                    chunk = self.get_chunk(chunk_x, chunk_y);

                    let cells = chunk.cells.borrow();
                    for x in needed.x1..needed.x2{
                        for y in needed.y1..needed.y2{
                            let cell_x = (x - chunk_rect.x1) as usize;
                            let cell_y = (y - chunk_rect.y1) as usize;
                            data[((x - rect.x1) + (((y - rect.y1) * rect_size.y))) as usize] = cells.get(cell_x, cell_y);
                        }
                    }
                }
            }
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
            let mut chunk = &self.root;
            let mut last_chunk = chunk;
            let mut last_move = skew_dir;
            // Skew
            for _ in 0..moves_skew {
                last_chunk = chunk;
                last_move = skew_dir;
                chunk = chunk.border[skew_dir].as_ref().unwrap();
            }
            // Horizontal
            for _ in 0..moves_hor {
                last_move = hor_dir;
                last_chunk = chunk;
                chunk = chunk.border[hor_dir].as_ref().unwrap();
            }
            // Vertical
            for _ in 0..moves_ver {
                last_move = ver_dir;
                last_chunk = chunk;
                chunk = chunk.border[ver_dir].as_ref().unwrap();
            }
            // Point from last chunk
            last_chunk.border[last_move].as_ref().unwrap().clone()
        }
    }
}
