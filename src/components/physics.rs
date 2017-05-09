use specs;

use def::Size;


/// 刚体组件，用于碰撞检测和其他物理操作。
///
/// 质量单位为 kg，碰撞范围为矩形，尺寸单位为像素。
pub struct RigidBody {
    mass: f32,
    size: Size,
}

impl RigidBody {
    pub fn new(mass: f32, w: u32, h: u32) -> RigidBody {
        RigidBody {
            mass,
            size: Size { w, h },
        }
    }
}

impl specs::Component for RigidBody {
    type Storage = specs::VecStorage<RigidBody>;
}
