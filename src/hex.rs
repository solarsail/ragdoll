use std::f64::consts::PI;
use std::ops::*;
use geometry::*;

type Mat2x2 = [[f64;2];2];

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub struct Hex {
	coord: [i32; 3]
}

const DIRECTIONS: [Hex; 6] = [
    Hex { coord: [ 1,-1, 0] },  // E
    Hex { coord: [ 1, 0,-1] },  // NE
    Hex { coord: [ 0, 1,-1] },  // NW
    Hex { coord: [-1, 1, 0] },  // W
    Hex { coord: [-1, 0, 1] },  // SW
    Hex { coord: [ 0,-1, 1] },  // SE
];

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

    pub fn edges(&self, layout: &Layout) -> [PointPair; 6] {
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
