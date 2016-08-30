extern crate piston_window;

use piston_window::{Context, G2d};
use piston_window::polygon::*;
//use piston_window::line::*;
use map::{Hex, Layout};


#[derive(Debug, PartialEq, Eq, Hash)]
pub struct MapCell {
    hex: Hex,
}

impl MapCell {
    pub fn new(q: i32, r: i32) -> MapCell {
        MapCell { hex: Hex::new(q, r) }
    }

    pub fn draw(&self, l: &Layout, c: Context, g: &mut G2d) {
        let fill = Polygon::new([0.5, 0.5, 0.5, 0.4]);
        fill.draw(&self.hex.vertices(l), &c.draw_state, c.transform, g);
        /*
        let border = Line::new([0.2, 0.2, 0.2, 1.0], 0.5);
        for edge in self.hex.edges_vertices(l).iter() {
            border.draw(edge, &DEFAULT_DRAW_STATE, c.transform, g);
        }
        */
    }
}
