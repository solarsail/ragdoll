use specs;

use def::Size;


/// 三维位置组件。实际画面中，高度由二维位移模拟。
///
/// 坐标单位为像素。
pub struct Position {
    pub x: i32,
    pub y: i32,
    pub z: i32,
}

impl Position {
    pub fn new(x: i32, y: i32, z: i32) -> Position {
        Position { x, y, z }
    }

    pub fn new2(x: i32, y: i32) -> Position {
        Position { x, y, z: 0 }
    }
}

impl specs::Component for Position {
    type Storage = specs::VecStorage<Position>;
}


/// 输入接收组件，作为接收输入的实体的标记。
///
/// 本身无内容，用于指示输入系统操作具备该组件的实体。
pub struct InputReceiver;

impl specs::Component for InputReceiver {
    type Storage = specs::VecStorage<InputReceiver>;
}


/// 渲染内容组件，提供贴图索引，贴图尺寸等。
///
/// 尺寸单位为像素。
#[derive(Debug)]
pub struct Renderable {
    tid: u32,
    size: Size,
}

impl Renderable {
    pub fn new(tid: u32, width: u32, height: u32) -> Renderable {
        Renderable {
            tid,
            size: Size { width, height },
        }
    }
}

impl specs::Component for Renderable {
    // TODO: 改为空间信息存储，便于剪切画面内容。
    type Storage = specs::VecStorage<Renderable>;
}


/// 刚体组件，用于碰撞检测和其他物理操作。
///
/// 质量单位为 kg，碰撞范围为矩形，尺寸单位为像素。
pub struct RigidBody {
    mass: f32,
    size: Size,
}

impl RigidBody {
    pub fn new(mass: f32, width: u32, height: u32) -> RigidBody {
        RigidBody {
            mass,
            size: Size { width, height },
        }
    }
}

impl specs::Component for RigidBody {
    type Storage = specs::VecStorage<RigidBody>;
}
