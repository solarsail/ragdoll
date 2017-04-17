use sdl2::render::Renderer;
use hexgrid::{Coordinates, Layout, POINTY_TOP, Point, PointPair};


#[derive(Debug, PartialEq, Eq, Hash)]
pub struct MapCell {
    coord: Coordinates,
}

impl MapCell {
    pub fn new(q: i32, r: i32) -> MapCell {
        MapCell { coord: Coordinates::at(q, r) }
    }

    pub fn draw(&self, l: &Layout, r: &mut Renderer) {
        // TODO
    }

    pub fn coordinates(&self) -> Coordinates {
        self.coord
    }
}
