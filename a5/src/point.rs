use std::cmp::{PartialEq, Eq};
use std::hash::{Hash, Hasher};

pub struct Point {
    x : f32,
    y : f32,
}

impl PartialEq for Point {
    fn eq(&self, other: &Point) -> bool {
        return other.x == self.x && other.y == self.y;
    }
}

impl Eq for Point {}

impl Hash for Point {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.x.to_string().hash(state);
        self.y.to_string().hash(state);
    }
}