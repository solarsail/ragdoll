use na::{Point2, Vector2};

#[derive(Debug)]
pub struct View {
    origin: Point2<i32>,
    width: u32,
    height: u32,
}

impl View {
    pub fn new() -> Self {
        View {
            origin: Point2::new(0, 0),
            width: 0,
            height: 0,
        }
    }

    pub fn set_size(&mut self, w: u32, h: u32) {
        self.width = w;
        self.height = h;
    }

    pub fn trans(&self, v: Vector2<i32>) -> Self {
        View {
            origin: self.origin + v,
            width: self.width,
            height: self.height,
        }
    }

    /*
    pub fn project(&self, rect: R) -> R
        where R: From<(i32, i32, u32, u32)> + Into<(i32, i32, u32, u32)> {

        let r = rect.into::<(i32, i32, u32, u32)>();

    }
    */

    pub fn project(&self, p: Point2<i32>) -> Point2<i32> {
        // TODO
        Point2::new(0, 0)
    }

    pub fn filter(&self, rect: (i32, i32, u32, u32)) -> bool {
        //overlap_rectangle(self.rect, rect).is_some()
        true
    }
}
