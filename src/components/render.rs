use specs;

use def::Size;
use sdl2::pixels::Color;


/// 渲染内容组件，提供贴图索引，贴图尺寸等。
///
/// 尺寸单位为像素。
#[derive(Debug)]
pub struct Renderable {
    pub tid: String,
    pub size: Size,
    pub alpha: u8,
}

impl Renderable {
    pub fn new<T: Into<String>>(tid: T, w: u32, h: u32) -> Renderable {
        Renderable::new_with_alpha(tid, w, h, 255)
    }

    pub fn new_with_alpha<T: Into<String>>(tid: T, w: u32, h: u32, alpha: u8) -> Renderable {
        Renderable {
            tid: tid.into(),
            size: Size { w, h },
            alpha,
        }
    }
}

impl specs::Component for Renderable {
    // TODO: 改为空间信息存储，便于剪切画面内容。
    type Storage = specs::VecStorage<Renderable>;
}

pub struct Text {
    pub fid: String,
    pub content: String,
    pub color: Color,
    pub max_width: u32,
}

impl Text {
    pub fn new<T: Into<String>>(fid: T, content: T, color: Color, max_width: u32) -> Text {
        Text {
            fid: fid.into(),
            content: content.into(),
            color,
            max_width,
        }
    }
}

impl specs::Component for Text {
    type Storage = specs::VecStorage<Text>;
}
