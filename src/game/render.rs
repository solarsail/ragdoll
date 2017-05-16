use std::collections::VecDeque;

use def::{Point, Size};


pub struct RenderCommand {
    pub texture_id: String,
    pub pos: Point,
    pub size: Size,
    pub alpha: u32,
}

pub struct RenderBuffer {
    pub tile_layer: VecDeque<RenderCommand>,
    pub object_layer: VecDeque<RenderCommand>,
}

impl RenderBuffer {
    pub fn new() -> RenderBuffer {
        RenderBuffer {
            tile_layer: VecDeque::new(),
            object_layer: VecDeque::new(),
        }
    }
}
