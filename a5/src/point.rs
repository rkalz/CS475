use std::cmp::{PartialEq, Eq};
use std::fmt;
use std::fmt::{Display, Formatter};
use std::hash::{Hash, Hasher};
use std::string::ToString;

#[derive(Copy, Clone)]
pub struct Point {
    pub x : f32,
    pub y : f32,
}

impl Point {
    pub fn new(_x: f32, _y: f32) -> Point {
        Point{ x: _x, y: _y }
    }
}

impl PartialEq for Point {
    fn eq(&self, other: &Point) -> bool {
        self.to_string().eq(&other.to_string())
    }
}

impl Eq for Point {}

impl Hash for Point {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.to_string().hash(state)
    }
}

impl Display for Point {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{:.4} {:.4}", self.y, self.x)
    }
}