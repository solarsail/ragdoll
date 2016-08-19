use std::rc::Rc;
use geometry::*;
use hex::*;

pub enum Type {
    Neutral,
    Friendly,
    Hostile,
    Player,
}

pub struct Region {
    rtype: Type,
    cells: Vec<Hex>,
}

impl Region {
    pub fn new() -> Region {
        Region {
            rtype: Type::Neutral,
            cells: vec![]
        }
    }

    pub fn push(&mut self, cell: MapCell) {
    }
}
