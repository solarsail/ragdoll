
#[derive(Debug)]
pub struct View {
	pub rect: [f64; 4],
	pub w2s_trans: Matrix2d,
	pub s2w_trans: Matrix2d,
}

impl View {
	pub fn new() -> Self {
		View {
			rect: [0.0; 4],
			w2s_trans: [
				[1.0, 0.0, 0.0],
				[0.0, 1.0, 0.0]
			],
			s2w_trans: [
				[1.0, 0.0, 0.0],
				[0.0, 1.0, 0.0]
			],
		}
	}

	pub fn set_size(&mut self, w: f64, h: f64) {
		self.rect[2] = w;
		self.rect[3] = h;
	}

	pub fn trans(&self, x: f64, y: f64) -> Self {
		View {
			rect: [
				self.rect[0]+x, self.rect[1]+y,
			    self.rect[2], self.rect[3]
			],
			w2s_trans: translate([-self.rect[0]-x, -self.rect[1]-y]), // world -> screen
			s2w_trans: translate([self.rect[0]+x, self.rect[1]+y])
		}
	}

	pub fn trans_self(&mut self, x: f64, y: f64) {
		self.w2s_trans[0][2] -= x;
		self.w2s_trans[1][2] -= y;
		self.s2w_trans[0][2] += x;
		self.s2w_trans[1][2] += y;
		self.rect[0] += x;
		self.rect[1] += y;
	}

	pub fn filter(&self, rect: [f64; 4]) -> bool {
		overlap_rectangle(self.rect, rect).is_some()
	}
}
