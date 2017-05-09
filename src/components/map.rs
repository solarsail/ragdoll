use specs;

use def::Size;


/// 地图组件，包含全部地形信息，由地形系统进行绘制。
///
/// TODO: 使用 serde 序列化和反序列化。
#[derive(Debug)]
pub struct Map {
    tiles: Vec<Tile>,
    size: Size,
}

impl Map {
    pub fn test(w: u32, h: u32) -> Map {
        Map {
            tiles: Vec::new(),
            size: Size { w, h },
        }
    }
}

impl specs::Component for Map {
    type Storage = specs::VecStorage<Map>;
}


/// 地图块类型枚举。
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum TileCategory {
    Grass,
    Soil,
}


/// 地图块。
///
/// TODO: 使用 serde 序列化和反序列化。
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct Tile {
    category: TileCategory,
    /// 同类型地形中的子类型编号，用于随机样式或
    /// 根据周围地形自适应。
    sub_id: u32,
}
