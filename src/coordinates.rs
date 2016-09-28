use amethyst::ecs::{VecStorage, Component};

use std::ops::*;

/// 2x2 矩阵，用于坐标变换。
type Mat2x2 = [[f64;2];2];

/// 六边形格坐标，使用立方体坐标系。
///
/// 对外表示时使用隐含 `s` 轴的二维坐标，表现为
/// ```
/// \
///  \
/// --*-- +q
///    \
///     \
///     +r
/// ```
#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub struct Coordinates {
    coord: [i32; 3]
}

impl Component for Coordinates {
    type Storage = VecStorage<Coordinates>;
}

/// 尖顶六边形的6个“接壤”方向的坐标向量，以边的方向定义，按逆时针方向排列。
const DIRECTIONS: [Coordinates; 6] = [
    Coordinates { coord: [ 1,-1, 0] },  // E
    Coordinates { coord: [ 1, 0,-1] },  // NE
    Coordinates { coord: [ 0, 1,-1] },  // NW
    Coordinates { coord: [-1, 1, 0] },  // W
    Coordinates { coord: [-1, 0, 1] },  // SW
    Coordinates { coord: [ 0,-1, 1] },  // SE
];

/// 尖顶六边形的6个“接壤”方向。
pub enum Direction {
    E = 0, NE, NW, W, SW, SE
}

impl Coordinates {
    pub fn new(q: i32, r: i32) -> Coordinates {
        Coordinates {
            coord: [q, -q-r, r]
        }
    }

    pub fn q(&self) -> i32 {
        self.coord[0]
    }

    pub fn r(&self) -> i32 {
        self.coord[2]
    }

    pub fn s(&self) -> i32 {
        self.coord[1]
    }

    pub fn length(&self) -> i32 {
        (self.q().abs() + self.r().abs() + self.s().abs()) / 2
    }

    pub fn distance(a: &Coordinates, b: &Coordinates) -> i32 {
        (a - b).length()
    }

    pub fn direction(d: Direction) -> Coordinates {
        let d = d as usize; 
        DIRECTIONS[d]
    }

    pub fn neighbour(&self, d: Direction) -> Coordinates {
        self + Coordinates::direction(d)
    }
}


impl Add for Coordinates {
    type Output = Coordinates;

    fn add(self, rhs: Coordinates) -> Coordinates {
        Coordinates::new(self.q() + rhs.q(), self.r() + rhs.r())
    }
}

impl<'a> Add<Coordinates> for &'a Coordinates {
    type Output = Coordinates;

    fn add(self, rhs: Coordinates) -> Coordinates {
        Coordinates::new(self.q() + rhs.q(), self.r() + rhs.r())
    }
}

impl<'a> Add<&'a Coordinates> for Coordinates {
    type Output = Coordinates;

    fn add(self, rhs: &Coordinates) -> Coordinates {
        Coordinates::new(self.q() + rhs.q(), self.r() + rhs.r())
    }
}

impl<'a, 'b> Add<&'a Coordinates> for &'b Coordinates {
    type Output = Coordinates;

    fn add(self, rhs: &'a Coordinates) -> Coordinates {
        Coordinates::new(self.q() + rhs.q(), self.r() + rhs.r())
    }
}

impl Sub for Coordinates {
    type Output = Coordinates;

    fn sub(self, rhs: Coordinates) -> Coordinates {
        Coordinates::new(self.q() - rhs.q(), self.r() - rhs.r())
    }
}

impl<'a> Sub<Coordinates> for &'a Coordinates {
    type Output = Coordinates;

    fn sub(self, rhs: Coordinates) -> Coordinates {
        Coordinates::new(self.q() - rhs.q(), self.r() - rhs.r())
    }
}

impl<'a> Sub<&'a Coordinates> for Coordinates {
    type Output = Coordinates;

    fn sub(self, rhs: &Coordinates) -> Coordinates {
        Coordinates::new(self.q() - rhs.q(), self.r() - rhs.r())
    }
}

impl<'a, 'b> Sub<&'a Coordinates> for &'b Coordinates {
    type Output = Coordinates;

    fn sub(self, rhs: &'a Coordinates) -> Coordinates {
        Coordinates::new(self.q() - rhs.q(), self.r() - rhs.r())
    }
}

impl Mul<i32> for Coordinates {
    type Output = Coordinates;

    fn mul(self, rhs: i32) -> Coordinates {
        Coordinates::new(self.q() * rhs, self.r() * rhs)
    }
}

impl<'a> Mul<i32> for &'a Coordinates {
    type Output = Coordinates;

    fn mul(self, rhs: i32) -> Coordinates {
        Coordinates::new(self.q() * rhs, self.r() * rhs)
    }
}

/*
pub struct Orientation {
    mat2screen: Mat2x2,
    mat2coord: Mat2x2,
    start_angle: f64 // * 60deg
}

pub const POINTY_TOP: Orientation = Orientation {
    mat2screen: [[SQRT3, SQRT3/2.0], [0.0, 1.5]],
    mat2coord:  [[SQRT3/3.0, -1.0/3.0], [0.0, 2.0/3.0]],
    start_angle: 0.5
};

#[allow(dead_code)]
pub const FLAT_TOP: Orientation = Orientation {
    mat2screen: [[1.5, 0.0], [SQRT3/2.0, SQRT3]],
    mat2coord:  [[2.0/3.0, 0.0], [-1.0/3.0, SQRT3/3.0]],
    start_angle: 0.0
};

pub struct Layout {
    orientation: Orientation,
    radius: Point,
    origin: Point
}

impl Layout {
    pub fn new(d: Orientation, r: Point, o: Point) -> Layout {
        Layout {
            orientation: d,
            radius: r,
            origin: o
        }
    }
}


#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub struct Edge {
    coord: [i32; 3]
}

impl Edge {
    pub fn new(q: i32, r: i32, p: i32) -> Edge {
        Edge {
            coord: [q, r, p]
        }
    }

    pub fn q(&self) -> i32 {
        self.coord[0]
    }

    pub fn r(&self) -> i32 {
        self.coord[1]
    }

    pub fn p(&self) -> usize {
        self.coord[2] as usize
    }

    pub fn vertices(&self, layout: &Layout) -> PointPair {
        let hex = Coordinates::new(self.q(), self.r());
        hex.edge_vertices_at(self.p(), layout)
    }
}
*/

