extern crate piston_window;

use std::collections::HashSet;
use std::cmp::{max, min};

use piston_window::{Context, G2d};
use piston_window::polygon::*;
//use piston_window::line::*;
use piston_window::draw_state::*;

use hex::*;
use default;


#[derive(Debug, PartialEq, Eq, Hash)]
struct MapCell {
    hex: Hex,
}

impl MapCell {
    pub fn new(q: i32, r: i32) -> MapCell {
        MapCell { hex: Hex::new(q, r) }
    }

    pub fn draw(&self, l: &Layout, c: Context, g: &mut G2d) {
        let fill = Polygon::new([0.5, 0.5, 0.5, 0.4]);
        fill.draw(&self.hex.vertices(l), default::draw_state(), c.transform, g);
        /*
        let border = Line::new([0.2, 0.2, 0.2, 1.0], 0.5);
        for edge in self.hex.edges_vertices(l).iter() {
            border.draw(edge, &DEFAULT_DRAW_STATE, c.transform, g);
        }
        */
    }
}



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

    pub fn draw(&self, l: &Layout, c: Context, g: &mut G2d) {
        for cell in self.content.iter() {
            cell.draw(l, c, g);
        }
    }
}
