use geometry::*;

#[derive(Debug)]
pub struct View {
	pub rect: [f64; 4],
	pub transform: Matrix2d,
}

impl View {
	pub fn new() -> Self {
		View {
			rect: [0.0; 4],
			transform: [
				[1.0, 0.0, 0.0],
				[0.0, 1.0, 0.0]
			],
		}
	}

	pub fn trans(&self, x: f64, y: f64) -> Self {
		let mut t = self.transform.clone();
		t[0][2] = self.transform[0][2]-x;
		t[1][2] = self.transform[1][2]-y;
		View {
			rect: [
				self.rect[0]+x, self.rect[1]+y,
			    self.rect[2], self.rect[3]
			],
			transform: t, // world -> screen
		}
	}

	pub fn filter<T: HasArea>(&self, obj: &T) -> bool {
		rect_intersect(self.rect, obj.bounding_box())
	}
}