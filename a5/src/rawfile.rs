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
        a + ((x - a) / (b - a))
    }

    pub fn build_and_write_isolines(&self, isoval: f32, path: &str) -> io::Result<()> {
        let mut vert_pos : HashMap<Point, usize> = HashMap::new();
        let mut verts : Vec<Point> = Vec::new();
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

                let mut pairs : Vec<(Point, Point)> = Vec::new();
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