use specs;
use na::{Point2, Vector2};

use def::{Point, Size};


/// 主摄像机组件。
pub struct MainCamera {
    origin: Point,
    size: Size,
}

impl MainCamera {
    pub fn new(x: i32, y: i32, w: u32, h: u32) -> MainCamera {
        MainCamera {
            origin: Point { x, y },
            size: Size { w, h },
        }
    }

    pub fn translate<V>(&mut self, v: V)
        where V: Into<Vector2<i32>>
    {
        let p: Point2<i32> = self.origin.into();
        self.origin = (p + v.into()).into();
    }

    pub fn origin(&self) -> Point {
        self.origin
    }

    pub fn size(&self) -> Size {
        self.size
    }
}

impl specs::Component for MainCamera {
    type Storage = specs::VecStorage<MainCamera>;
}
