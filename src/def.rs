use cgmath::{Point2, Vector2};


pub type Point = Point2<f32>;
pub type Vector = Vector2<f32>;


#[derive(Debug, Clone, Copy)]
pub struct Size {
    pub w: u32,
    pub h: u32,
}

impl Size {
    pub fn new(w: u32, h: u32) -> Size {
        Size { w, h }
    }
}
