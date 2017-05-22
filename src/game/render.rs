use std::collections::VecDeque;
use std::convert::{From, Into};

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


pub struct ScreenDimension {
    pub w: u32,
    pub h: u32,
}

impl ScreenDimension {
    pub fn new(w: u32, h: u32) -> ScreenDimension {
        ScreenDimension { w, h }
    }
}

impl Into<Size> for ScreenDimension {
    fn into(self) -> Size {
        Size {
            w: self.w,
            h: self.h,
        }
    }
}

impl From<Size> for ScreenDimension {
    fn from(s: Size) -> ScreenDimension {
        ScreenDimension { w: s.w, h: s.h }
    }
}
