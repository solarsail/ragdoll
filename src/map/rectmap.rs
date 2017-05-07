use std::collections::HashSet;
use std::cmp::{max, min};

use sdl2::render::Renderer;

use rectgrid::Layout;
use map::mapcell::MapCell;
use view::View;
use game::GameContext;


pub struct RectMap {
    width: u32,
    height: u32,
    content: Vec<MapCell>,
}

impl RectMap {
    pub fn test(radius: u32) -> RectMap {
        let mut map = RectMap {
            width: radius * 2,
            height: radius * 2,
            content: Vec::new(),
        };
        for i in -(radius as i32)..radius as i32 {
            for j in -(radius as i32)..radius as i32 {
                map.content.push(MapCell::new(i, j));
            }
        }
        map
    }

    #[allow(dead_code)]
    pub fn width(&self) -> u32 {
        self.width
    }

    #[allow(dead_code)]
    pub fn height(&self) -> u32 {
        self.height
    }

    pub fn draw(&self, ctx: &mut GameContext, l: &Layout, v: &View, r: &mut Renderer) {
        for cell in self.content
                .iter()
                .filter(|c| v.filter(l.bounding_box_of(c.tile()))) {
            // TODO
            cell.draw(ctx, l, r);
        }
    }
}
