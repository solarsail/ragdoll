use std::collections::VecDeque;

use sdl2::pixels::Color;
use def::{Point, Size};


pub enum RenderCommand {
    Texture {
        texture_id: String,
        pos: Point,
        size: Option<Size>,
        alpha: Option<u8>,
    },
    Text {
        font_id: String,
        content: String,
        width: u32,
        color: Color,
        pos: Point,
    },
}

pub type RenderBuffer0 = VecDeque<RenderCommand>;
pub type RenderBuffer1 = VecDeque<RenderCommand>;
