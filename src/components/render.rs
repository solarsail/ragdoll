use specs;

use def::Size;


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
