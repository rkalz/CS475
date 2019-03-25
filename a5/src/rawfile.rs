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

    fn check_if_duo_in_bounds(x: f32, y: f32) -> bool {
        x > 0.0 && x < 1.0 && y > 0.0 && y < 1.0
    }

    fn check_if_all_in_bounds(w: f32, x: f32, y: f32, z: f32) -> bool {
        RawFile::check_if_duo_in_bounds(w, x) && RawFile::check_if_duo_in_bounds(y, z)
    }

    pub fn build_and_write_isolines(&self, isoval: f32, path: &str) -> io::Result<()> {
        let mut pairs : Vec<(Point, Point)> = Vec::new();

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

                let row_one_f = row_one as f32;
                let col_one_f = col_one as f32;
                let row_two_f = row_two as f32;
                let col_two_f = col_two as f32;

                let lpx_offset = RawFile::get_inter_pos(isoval, top_left_val, bot_left_val);
                let left_point = Point::new(row_one_f + lpx_offset, col_one_f);

                let tpy_offset = RawFile::get_inter_pos(isoval, top_left_val, top_right_val);
                let top_point = Point::new(row_one_f, col_one_f + tpy_offset);

                let bpy_offset = RawFile::get_inter_pos(isoval, bot_left_val, bot_right_val);
                let bot_point = Point::new(row_two_f, col_one_f + bpy_offset);

                let rpx_offset = RawFile::get_inter_pos(isoval, top_right_val, bot_right_val);
                let right_point = Point::new(row_two_f + (1.0 - rpx_offset), col_two_f);

                if top_left && top_right && bot_right {
                    if !RawFile::check_if_duo_in_bounds(lpx_offset, bpy_offset) {continue;}

                    pairs.push((left_point, bot_point)) // Case 14
                } else if top_left && top_right && bot_left {
                    if !RawFile::check_if_duo_in_bounds(bpy_offset, rpx_offset) {continue;}

                    pairs.push((bot_point, right_point)) // Case 13
                } else if top_left && top_right {
                    if !RawFile::check_if_duo_in_bounds(lpx_offset, rpx_offset) {continue;}

                    pairs.push((left_point, right_point)) // Case 12
                } else if top_left && bot_left && bot_right {
                    if !RawFile::check_if_duo_in_bounds(tpy_offset, rpx_offset) {continue;}

                    pairs.push((top_point, right_point)) // Case 11
                } else if top_left && bot_right {
                    if !RawFile::check_if_all_in_bounds(tpy_offset, rpx_offset,
                         lpx_offset, bpy_offset) {continue;}

                    pairs.push((top_point, right_point)); // Case 10
                    pairs.push((left_point, bot_point))
                } else if top_left && bot_left {
                    if !RawFile::check_if_duo_in_bounds(tpy_offset, bpy_offset) {continue;}

                    pairs.push((top_point, bot_point)) // Case 9
                } else if top_left {
                    if !RawFile::check_if_duo_in_bounds(lpx_offset, tpy_offset) {continue;}

                    pairs.push((left_point, top_point)) // Case 8
                } else if top_right && bot_left && bot_right {
                    if !RawFile::check_if_duo_in_bounds(lpx_offset, tpy_offset) {continue;}

                    pairs.push((left_point, top_point)) // Case 7
                } else if top_right && bot_right {
                    if !RawFile::check_if_duo_in_bounds(tpy_offset, bpy_offset) {continue;}

                    pairs.push((top_point, bot_point)) // Case 6
                } else if top_right && bot_left {
                    if !RawFile::check_if_all_in_bounds(tpy_offset, rpx_offset,
                         lpx_offset, bpy_offset) {continue;}

                    pairs.push((left_point, top_point)); // Case 5
                    pairs.push((bot_point, right_point))
                } else if top_right {
                    if !RawFile::check_if_duo_in_bounds(tpy_offset, rpx_offset) {continue;}

                    pairs.push((top_point, right_point)) // Case 4
                } else if bot_left && bot_right {
                    if !RawFile::check_if_duo_in_bounds(lpx_offset, rpx_offset) {continue;}

                    pairs.push((left_point, right_point)) // Case 3
                } else if bot_right {
                    if !RawFile::check_if_duo_in_bounds(bpy_offset, rpx_offset) {continue;}

                    pairs.push((bot_point, right_point)) // Case 2
                } else if bot_left {
                    if !RawFile::check_if_duo_in_bounds(lpx_offset, bpy_offset) {continue;}

                    pairs.push((left_point, bot_point)) // Case 1
                }
            }
        }

        let mut unique_points : HashMap<Point, usize> = HashMap::new();
        let mut verts : Vec<Point> = Vec::new();
        let mut edges : Vec<(usize, usize)> = Vec::new();

        for pair in pairs {
            if !unique_points.contains_key(&pair.0) {
                verts.push(pair.0);
                unique_points.insert(pair.0, verts.len());
            }
            if !unique_points.contains_key(&pair.1) {
                verts.push(pair.1);
                unique_points.insert(pair.1, verts.len());
            }

            let a_pos = *unique_points.get(&pair.0).unwrap();
            let b_pos = *unique_points.get(&pair.1).unwrap();

            edges.push((a_pos.min(b_pos), a_pos.max(b_pos)));
        }

        let mut f = File::create(path)?;
        f.write(b"# Generated by Rofael Aleezada for CS 475: Homework 5\n")?;
        for vert in verts {
            f.write_fmt(format_args!("v {0} 0\n", vert))?;
        }
        for edge in edges {
            f.write_fmt(format_args!("l {0} {1}\n", edge.0, edge.1))?;
        }
        Ok(())
    }

}