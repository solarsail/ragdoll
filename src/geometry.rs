extern crate piston_window;

use piston_window::math::Vec2d;
use std::convert::Into;

pub use piston_window::math::add;
pub type Point = Vec2d<f64>;

#[derive(Debug, Clone, Copy)]
pub struct PointPair {
    pair: [Point; 2],
}

impl PointPair {
    pub fn new(a: Point, b: Point) -> PointPair {
        PointPair { pair: [a, b] }
    }

    pub fn set(&mut self, i: usize, p: Point) {
        self.pair[i] = p;
    }
}

impl Into<[f64; 4]> for PointPair {
    fn into(self) -> [f64; 4] {
        [self.pair[0][0], self.pair[0][1], self.pair[1][0], self.pair[1][1]]
    }
}

impl<'a> Into<[f64; 4]> for &'a PointPair {
    fn into(self) -> [f64; 4] {
        [self.pair[0][0], self.pair[0][1], self.pair[1][0], self.pair[1][1]]
    }
}

pub fn neg(vec: Point) -> Point {
    [-vec[0], -vec[1]]
}

pub const SQRT3: f64 = 1.7320508;
