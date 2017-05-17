use std::convert::{From, Into};
use na::Point2;


#[derive(Debug, Clone, Copy)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

impl Point {
    pub fn new(x: i32, y: i32) -> Point {
        Point { x, y }
    }
}

impl Into<Point2<i32>> for Point {
    fn into(self) -> Point2<i32> {
        Point2::new(self.x, self.y)
    }
}

impl From<Point2<i32>> for Point {
    fn from(p: Point2<i32>) -> Point {
        Point {
            x: p.coords[0],
            y: p.coords[1],
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Size {
    pub w: u32,
    pub h: u32,
}

impl Size {
    pub fn new(w: u32, h: u32) -> Size {
        Size { w, h }
    }
}
