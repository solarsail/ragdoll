use specs;


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
