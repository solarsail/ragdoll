extern crate piston_window;

use std::convert::Into;

/// 向量加法。
pub use piston_window::math::add;
pub type Point = [f64; 2];

/// 点对，可以用于表示线段的端点。
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

/// 向量取反。
#[allow(dead_code)]
pub fn neg(vec: Point) -> Point {
    [-vec[0], -vec[1]]
}

/// 3 的平方根。
pub const SQRT3: f64 = 1.7320508;
