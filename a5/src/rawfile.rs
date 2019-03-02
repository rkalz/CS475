use std::collections::HashMap;
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
        return Result::Ok(self.buf[pos]);
    }

    pub fn build_and_write_isolines(&self, isoval: f32, path: &str) -> io::Result<()> {
        let mut verts : HashMap<Point, usize> = HashMap::new();
        let mut edges : Vec<(usize, usize)> = Vec::new();
        for i in 0..self.height {
            for j in 0..self.width {
                let a_val = match self.get_value(i, j) {
                    Ok(v) => v as f32,
                    Err(_) => continue
                };
                let b_val = match self.get_value(i, j + 1) {
                    Ok(v) => v as f32,
                    Err(_) => continue
                };
                let c_val = match self.get_value(i + 1, j) {
                    Ok(v) => v as f32,
                    Err(_) => continue
                };
                let d_val = match self.get_value(i + 1, j + 1) {
                    Ok(v) => v as f32,
                    Err(_) => continue
                };

                let a = a_val >= isoval;
                let b = b_val >= isoval;
                let c = c_val >= isoval;
                let d = d_val >= isoval;

                if !a && !b && c && !d {
                    // Case 1
                } if !a && !b && !c && d {
                    // Case 2
                } if !a && !b && c && d {
                    // Case 3
                } if !a && b && !c && !d {
                    // Case 4
                } if !a && b && c && !d {
                    // Case 5
                } if !a && b && !c && d {
                    // Case 6
                } if !a && b && c && d {
                    // Case 7
                } if a && !b && !c && !d {
                    // Case 8
                } if a && !b && c && !d {
                    // Case 9
                } if a && !b && !c && d {
                    // Case 10
                } if a && !b && c && d {
                    // Case 11
                } if a && b && !c && !d {
                    // Case 12
                } if a && b && c && !d {
                    // Case 13
                } if a && b && !c && d {
                    // Case 14
                }
            }
        }

        Ok(())
    }

}