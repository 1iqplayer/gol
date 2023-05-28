use std::cmp::{max, min};

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
    pub fn size(&self) -> Vec2<i64> {
        Vec2::new(self.x2 - self.x1, self.y2 - self.y1)
    }
    pub fn intersect(&self, r2: &Vec4<i64>) -> Option<Vec4<i64>> {
        let rect = Vec4 {
            x1: max(self.x1, r2.x1),
            y1: max(self.y1, r2.y1),
            x2: min(self.x2, r2.x2),
            y2: min(self.y2, r2.y2),
        };
        if rect.x1 == rect.x2 || rect.y1 == rect.y2 {
            return None;
        }
        Some(rect)
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
pub fn dir2index(x: i64, y: i64) -> usize {
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
