use sdl2::rect::Rect;
use sdl2::render::Renderer;

use rectgrid::{Tile, Position, Layout};
use game::GameContext;


#[derive(Debug, PartialEq, Eq, Hash)]
pub struct MapCell {
    tile: Tile,
}

impl MapCell {
    pub fn new(x: i32, y: i32) -> MapCell {
        MapCell { tile: Tile::new(x, y) }
    }

    pub fn draw(&self, ctx: &mut GameContext, l: &Layout, r: &mut Renderer) {
        let rect = l.bounding_box_of(self.tile);
        let dest_rect = Rect::new(rect.0, rect.1, rect.2, rect.3);
        r.copy(ctx.res.rect_texture(), None, Some(dest_rect))
            .unwrap();
    }

    pub fn tile(&self) -> Tile {
        self.tile
    }
}
