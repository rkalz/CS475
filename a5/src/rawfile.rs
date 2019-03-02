use std::collections::*;
use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::f32;
use std::result::Result;

use crate::point::Point;

pub struct RawFile {
    buf : Vec<u8>,
    width : usize,
    height : usize,
}

impl RawFile {
    pub fn new(path: &str, w: usize, h: usize) -> io::Result<RawFile> {
        let mut f = File::open(path)?;
        let mut buffer = Vec::new();
        f.read_to_end(&mut buffer)?;

        Ok(RawFile {buf: buffer, width: w, height: h})
    }

    fn get_value(&self, i: usize, j: usize) -> Result<u8, ()> {
        let pos = i * self.height + j;
        if pos >= self.buf.len() {
            return Result::Err(());
        }
        Result::Ok(self.buf[pos])
    }

    fn get_inter_pos(x: f32, a: f32, b: f32) -> f32 {
        if a == b {
            return a;
        }
        if a > b {
            return RawFile::get_inter_pos(x, b, a);
        }
        (x - a) / (b - a)
    }

    pub fn build_and_write_isolines(&self, isoval: f32, path: &str) -> io::Result<()> {
        let mut vert_pos : HashMap<Point, usize> = HashMap::new();
        let mut verts : Vec<Point> = Vec::new();
        let mut edges : Vec<(usize, usize)> = Vec::new();
        for row_one in 0..self.height{
            let row_two = row_one + 1;
            for col_one in 0..self.width {
                let col_two = col_one + 1;

                let top_left_val = match self.get_value(row_one, col_one) {
                    Ok(v) => v as f32,
                    Err(_) => continue
                };
                let top_right_val = match self.get_value(row_one, col_two) {
                    Ok(v) => v as f32,
                    Err(_) => continue
                };
                let bot_left_val = match self.get_value(row_two, col_one) {
                    Ok(v) => v as f32,
                    Err(_) => continue
                };
                let bot_right_val = match self.get_value(row_two, col_two) {
                    Ok(v) => v as f32,
                    Err(_) => continue
                };

                let top_left = top_left_val >= isoval;
                let top_right = top_right_val >= isoval;
                let bot_left = bot_left_val >= isoval;
                let bot_right = bot_right_val >= isoval;

                let mut pairs : Vec<(Point, Point)> = Vec::new();
                if bot_left && !(top_left || top_right || bot_right) {
                    // Case 1
                    let x_one = (row_one as f32) + RawFile::get_inter_pos(isoval, top_left_val, bot_left_val);
                    let y_two = (col_one as f32) + RawFile::get_inter_pos(isoval, bot_left_val, bot_right_val);

                    pairs.push((Point::new(x_one, col_one as f32), Point::new(row_two as f32, y_two)));
                }
                if bot_right && !(top_left || top_right || bot_left) {
                    // Case 2
                    let x_two = (row_one as f32) + RawFile::get_inter_pos(isoval, top_right_val, bot_right_val);
                    let y_one = (col_one as f32) + RawFile::get_inter_pos(isoval, bot_left_val, bot_right_val);

                    pairs.push((Point::new(row_two as f32, y_one), Point::new(x_two, col_two as f32)));
                }

                for pair in pairs {
                    if !vert_pos.contains_key(&pair.0) {
                        verts.push(pair.0);
                        vert_pos.insert(pair.0, verts.len());
                    } if !vert_pos.contains_key(&pair.1) {
                        verts.push(pair.1);
                        vert_pos.insert(pair.1, verts.len());
                    }

                    let a = *vert_pos.get(&pair.0).unwrap();
                    let b = *vert_pos.get(&pair.1).unwrap();

                    edges.push((a, b));
                }
            }
        }

        let mut f = File::create(path)?;
        for vert in verts {
            f.write_fmt(format_args!("v {0} {1} 0\n", vert.y, vert.x))?;
        }
        for edge in edges {
            f.write_fmt(format_args!("l {0} {1}\n", edge.0, edge.1))?;
        }
        Ok(())
    }

}