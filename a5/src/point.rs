use std::string::ToString;

pub struct Point {
    pub x : f32,
    pub y : f32,
}

impl Point {
    pub fn new(_x: f32, _y: f32) -> Point {
        Point{ x: _x, y: _y }
    }
}

impl ToString for Point {
    fn to_string(&self) -> String {
        format!("{:.4} {:.4}", self.y, self.x)
    }
}