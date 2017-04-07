extern crate piston_window;

use std::collections::HashSet;
use std::cmp::{max, min};

use piston_window::{Context, G2d};

use map::Layout;
use map::mapcell::MapCell;
use view::View;


pub struct HexMap {
    radius: i32,
    content: HashSet<MapCell>,
}

impl HexMap {
    pub fn new(radius: i32) -> HexMap {
        let mut map = HexMap {
            radius: radius,
            content: HashSet::new()
        };
        for q in -radius..radius+1 {
            let r1 = max(-radius, -radius - q);
            let r2 = min(radius, radius - q);
            for r in r1..r2+1 {
                map.content.insert(MapCell::new(q, r));
            }
        }
        map
    }

    #[allow(dead_code)]
    pub fn radius(&self) -> i32 {
        self.radius
    }

    pub fn draw(&self, l: &Layout, v: &View, c: Context, g: &mut G2d) {
        for cell in self.content.iter().filter(|c| v.filter(c.bounding_box(l))) {
            cell.draw(l, c, g);
        }
    }
}