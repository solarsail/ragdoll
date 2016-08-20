use std::f64::consts::PI;
use std::ops::*;
use geometry::*;

/// 2x2 矩阵，用于坐标变换。
type Mat2x2 = [[f64;2];2];

/// 六边形格坐标，使用立方体坐标系。
///
/// ```
/// +s    +q
///   \ | /
///    \|/
///     *
///    /|\
///   / | \
///    +r
/// ```
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
pub struct Hex {
	coord: [i32; 3]
}

/// 尖顶六边形的6个“接壤”方向的坐标向量，以边的方向定义，按逆时针方向排列。
const DIRECTIONS: [Hex; 6] = [
    Hex { coord: [ 1,-1, 0] },  // E
    Hex { coord: [ 1, 0,-1] },  // NE
    Hex { coord: [ 0, 1,-1] },  // NW
    Hex { coord: [-1, 1, 0] },  // W
    Hex { coord: [-1, 0, 1] },  // SW
    Hex { coord: [ 0,-1, 1] },  // SE
];

/// 尖顶六边形的6个“接壤”方向。
pub enum Direction {
    E = 0, NE, NW, W, SW, SE
}

impl Hex {
    pub fn new(q: i32, r: i32) -> Hex {
    	Hex {
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

    pub fn distance(a: &Hex, b: &Hex) -> i32 {
    	(a - b).length()
    }

    pub fn direction(d: Direction) -> Hex {
        let d = d as usize; 
        DIRECTIONS[d]
    }

    pub fn neighbour(&self, d: Direction) -> Hex {
        self + Hex::direction(d)
    }

    pub fn center_pixel(&self, layout: &Layout) -> Point {
        let mat = layout.orientation.mat2screen;
        let radius = layout.radius;
        let origin = layout.origin;
        let x = (mat[0][0] * self.q() as f64 + mat[0][1] * self.r() as f64) * radius[0];
        let y = (mat[1][0] * self.q() as f64 + mat[1][1] * self.r() as f64) * radius[1];
        [x + origin[0], y + origin[1]]
    }

    /// 定点相对于中心的距离向量。
    /// ```
    ///   4 *
    /// 3 *   * 5
    /// 2 *   * 0
    ///     * 1
    /// ```
    fn vertex_offset(layout: &Layout, index: usize) -> Point {
        let radius = layout.radius;
        let angle = PI * (layout.orientation.start_angle + index as f64) / 3.0;
        [radius[0] * angle.cos(), radius[1] * angle.sin()]
    }

    pub fn vertices(&self, layout: &Layout) -> [Point; 6] {
        let mut vertices = [[0.0, 0.0]; 6];
        let center = self.center_pixel(layout);
        for i in 0..6 {
            let offset = Hex::vertex_offset(layout, i);
            vertices[i] = add(center, offset);
        }
        vertices
    }

    pub fn edge_vertices_at(&self, p: usize, layout: &Layout) -> PointPair {
        debug_assert!(p < 3, "invalid edge index {}", p);
        let center = self.center_pixel(layout);
        let v1 = add(center, Hex::vertex_offset(layout, p));
        let v2 = add(center, Hex::vertex_offset(layout, p+1));
        PointPair::new(v1, v2)
    }

    pub fn edges_vertices(&self, layout: &Layout) -> [PointPair; 6] {
        let mut edges = [PointPair::new([0.0, 0.0], [0.0, 0.0]); 6];
        let center = self.center_pixel(layout);
        for i in 0..6 {
            let offset = Hex::vertex_offset(layout, i);
            let p = add(center, offset);
            edges[i].set(0, p);
            edges[(i+5) % 6].set(1, p);
        }
        edges
    }

    pub fn edges(&self) -> [Edge; 6] {
        [Edge::new(self.q(), self.r(), 0),
         Edge::new(self.q(), self.r(), 1),
         Edge::new(self.q(), self.r(), 2),
         Edge::new(self.q(), self.r()-1, 0),
         Edge::new(self.q()+1, self.r()-1, 1),
         Edge::new(self.q()+1, self.r(), 2)]
    }

    fn round(fq: f64, fr: f64, fs:f64) -> Hex {
        let mut q = fq.round();
        let mut r = fr.round();
        let mut s = fs.round();
        let dq = (q - fq).abs();
        let dr = (r - fr).abs();
        let ds = (s - fs).abs();
        if dq > dr && dq > ds {
            q = -r - s;
        } else if dr > ds {
            r = -q - s;
        } else {
            s = -q - r;
        }
        Hex { coord: [q as i32, r as i32, s as i32] }
    }

    pub fn pixel2hex(p: Point, layout: &Layout) -> Hex {
        let mat = layout.orientation.mat2coord;
        let origin = layout.origin;
        let radius = layout.radius;
        let pt = [(p[0] - origin[0]) / radius[0], (p[1] - origin[1]) / radius[1]];
        let q = mat[0][0] * pt[0] + mat[0][1] * pt[1];
        let r = mat[1][0] * pt[0] + mat[1][1] * pt[1];
        Hex::round(q, r, -q-r)
    }
}

impl Add for Hex {
    type Output = Hex;

    fn add(self, rhs: Hex) -> Hex {
        Hex::new(self.q() + rhs.q(), self.r() + rhs.r())
    }
}

impl<'a> Add<Hex> for &'a Hex {
    type Output = Hex;

    fn add(self, rhs: Hex) -> Hex {
        Hex::new(self.q() + rhs.q(), self.r() + rhs.r())
    }
}

impl<'a> Add<&'a Hex> for Hex {
    type Output = Hex;

    fn add(self, rhs: &Hex) -> Hex {
        Hex::new(self.q() + rhs.q(), self.r() + rhs.r())
    }
}

impl<'a, 'b> Add<&'a Hex> for &'b Hex {
    type Output = Hex;

    fn add(self, rhs: &'a Hex) -> Hex {
        Hex::new(self.q() + rhs.q(), self.r() + rhs.r())
    }
}

impl Sub for Hex {
    type Output = Hex;

    fn sub(self, rhs: Hex) -> Hex {
        Hex::new(self.q() - rhs.q(), self.r() - rhs.r())
    }
}

impl<'a> Sub<Hex> for &'a Hex {
    type Output = Hex;

    fn sub(self, rhs: Hex) -> Hex {
        Hex::new(self.q() - rhs.q(), self.r() - rhs.r())
    }
}

impl<'a> Sub<&'a Hex> for Hex {
    type Output = Hex;

    fn sub(self, rhs: &Hex) -> Hex {
        Hex::new(self.q() - rhs.q(), self.r() - rhs.r())
    }
}

impl<'a, 'b> Sub<&'a Hex> for &'b Hex {
    type Output = Hex;

    fn sub(self, rhs: &'a Hex) -> Hex {
        Hex::new(self.q() - rhs.q(), self.r() - rhs.r())
    }
}

impl Mul<i32> for Hex {
    type Output = Hex;

    fn mul(self, rhs: i32) -> Hex {
        Hex::new(self.q() * rhs, self.r() * rhs)
    }
}

impl<'a> Mul<i32> for &'a Hex {
    type Output = Hex;

    fn mul(self, rhs: i32) -> Hex {
        Hex::new(self.q() * rhs, self.r() * rhs)
    }
}

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
        let hex = Hex::new(self.q(), self.r());
        hex.edge_vertices_at(self.p(), layout)
    }
}
