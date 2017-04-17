use na::geometry::{Point2, Vector2};

#[derive(Debug)]
pub struct View {
    origin: Point2<f64>,
    width: f64,
    height: f64,
}

impl View {
    pub fn new() -> Self {
        View {
            origin: Point2::new(0.0, 0.0),
            width: 0,
            height: 0,
        }
    }

    pub fn set_size(&mut self, w: f64, h: f64) {
        self.width = w;
        self.height = h;
    }

    pub fn trans(&self, v: Vector2) -> Self {
        View {
            origin: self.origin + v,
            width: self.width,
            height: self.height,
        }
    }

    pub fn project(&self, p: Point2<f64>) -> Point2<f64> {
        // TODO
        Point2::new(0.0, 0.0)
    }

    pub fn filter(&self, rect: [f64; 4]) -> bool {
        //overlap_rectangle(self.rect, rect).is_some()
        true
    }
}
