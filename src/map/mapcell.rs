use sdl2::rect::Rect;
use sdl2::render::Renderer;

use hexgrid::{Coordinates, Layout, POINTY_TOP, Point, PointPair};
use game::GameContext;


#[derive(Debug, PartialEq, Eq, Hash)]
pub struct MapCell {
    coord: Coordinates,
}

impl MapCell {
    pub fn new(q: i32, r: i32) -> MapCell {
        MapCell { coord: Coordinates::at(q, r) }
    }

    pub fn draw(&self, ctx: &mut GameContext, l: &Layout, r: &mut Renderer) {
        let rect = l.bounding_box_of(self.coord);
        let dest_rect = Rect::new(rect[0] as i32, rect[1] as i32, rect[2] as u32, rect[3] as u32);
        r.copy(ctx.res.hex_texture(), None, Some(dest_rect)).unwrap();
    }

    pub fn coordinates(&self) -> Coordinates {
        self.coord
    }
}
