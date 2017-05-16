use specs;

use def::Size;


/// 渲染内容组件，提供贴图索引，贴图尺寸等。
///
/// 尺寸单位为像素。
#[derive(Debug)]
pub struct Renderable {
    tid: String,
    pub alpha: u32,
    size: Size,
}

impl Renderable {
    pub fn new<T: Into<String>>(tid: T, alpha: u32, w: u32, h: u32) -> Renderable {
        Renderable {
            tid: tid.into(),
            alpha,
            size: Size { w, h },
        }
    }
}

impl specs::Component for Renderable {
    // TODO: 改为空间信息存储，便于剪切画面内容。
    type Storage = specs::VecStorage<Renderable>;
}
