/// 3 的平方根。
pub const SQRT3: f32 = 1.7320508;pub type Point = [f32; 2];

/// 点对，可以用于表示线段的端点。
#[allow(dead_code)]
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

impl Into<[f32; 4]> for PointPair {
    fn into(self) -> [f32; 4] {
        [self.pair[0][0], self.pair[0][1], self.pair[1][0], self.pair[1][1]]
    }
}

impl<'a> Into<[f32; 4]> for &'a PointPair {
    fn into(self) -> [f32; 4] {
        [self.pair[0][0], self.pair[0][1], self.pair[1][0], self.pair[1][1]]
    }
}

/// 向量取反。
#[allow(dead_code)]
pub fn neg(vec: Point) -> Point {
    [-vec[0], -vec[1]]
}

