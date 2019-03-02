use std::cmp::{PartialEq, Eq};
use std::hash::{Hash, Hasher};

#[derive(Clone, Copy)]
pub struct Point {
    pub x : f32,
    pub y : f32,
}

impl Point {
    pub fn new(_x : f32, _y : f32) -> Point {
        Point{ x: _x, y: _y }
    }
}

impl PartialEq for Point {
    fn eq(&self, other: &Point) -> bool {
        other.x == self.x && other.y == self.y
    }
}

impl Eq for Point {}

impl Hash for Point {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.x.to_string().hash(state);
        self.y.to_string().hash(state);
    }
}