use piston_window::{Context, G2d};
use piston_window::polygon::*;
//use piston_window::line::*;
use hexgrid::{Coordinates, Layout, POINTY_TOP, Point, PointPair};


#[derive(Debug, PartialEq, Eq, Hash)]
pub struct MapCell {
    coord: Coordinates,
}

impl MapCell {
    pub fn new(q: i32, r: i32) -> MapCell {
        MapCell { coord: Coordinates::at(q, r) }
    }

    pub fn draw(&self, l: &Layout, c: Context, g: &mut G2d) {
        let fill = Polygon::new([0.5, 0.5, 0.5, 0.4]);
        fill.draw(&l.vertices_of_hex(self.coord).iter().map(|p| p.into()).collect::<Vec<[f64;2]>>(), &c.draw_state, c.transform, g);
        /*
        let border = Line::new([0.2, 0.2, 0.2, 1.0], 0.5);
        for edge in layout.all_edges_of_hex(self.coord).iter() {
            border.draw(edge, &DEFAULT_DRAW_STATE, c.transform, g);
        }
        */
    }

    pub fn coordinates(&self) -> Coordinates {
        self.coord
    }
}
