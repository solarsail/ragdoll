use specs;


/// 三维位置组件。实际画面中，高度由二维位移模拟。
///
/// 坐标单位为像素。
pub struct Position {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Position {
    pub fn new(x: f32, y: f32, z: f32) -> Position {
        Position { x, y, z }
    }

    pub fn new2(x: f32, y: f32) -> Position {
        Position { x, y, z: 0.0 }
    }
}

impl specs::Component for Position {
    type Storage = specs::VecStorage<Position>;
}
