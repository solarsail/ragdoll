use specs;

use def::{Point, Size};


/// 主摄像机组件。
pub struct MainCamera {
    pub size: Size,
}

impl MainCamera {
    pub fn new(w: u32, h: u32) -> MainCamera {
        MainCamera { size: Size { w, h } }
    }

    pub fn size(&self) -> Size {
        self.size
    }
}

impl specs::Component for MainCamera {
    type Storage = specs::VecStorage<MainCamera>;
}
