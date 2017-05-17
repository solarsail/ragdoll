use std::collections::VecDeque;
use std::cell::RefCell;

use def::{Point, Size};


pub struct RenderCommand {
    pub texture_id: String,
    pub pos: Point,
    pub size: Size,
    pub alpha: u8,
}

pub type RenderBuffer_0 = VecDeque<RenderCommand>;
pub type RenderBuffer_1 = VecDeque<RenderCommand>;
