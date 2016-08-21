extern crate piston_window;

use std::collections::HashSet;

use piston_window::{Context, G2d};
use piston_window::polygon::*;
use piston_window::line::*;
use piston_window::draw_state::*;

use hex::*;


const DEFAULT_DRAW_STATE: DrawState = DrawState {
    scissor: None,
    stencil: None,
    blend: Some(Blend::Alpha)
};

pub enum Category {
    Neutral,
    Friendly,
    Hostile,
    Player,
}

fn type_color(category: &Category) -> [f32; 4] {
    match category {
        &Category::Neutral => [0.8, 0.8, 0.0, 0.5],
        &Category::Friendly => [0.0, 1.0, 0.0, 0.5],
        &Category::Hostile => [1.0, 0.0, 0.0, 0.5],
        &Category::Player => [0.0, 0.0, 1.0, 0.5]
    }
}

pub struct Region {
    category: Category,
    cells: HashSet<Hex>,
}

impl Region {
    pub fn new() -> Region {
        Region {
            category: Category::Neutral,
            cells: HashSet::new()
        }
    }

    pub fn category(mut self, t: Category) -> Self {
        self.category = t;
        self
    }

    pub fn push(&mut self, hex: Hex) {
        self.cells.insert(hex);
    }

    pub fn clear(&mut self) {
        self.cells.clear();
    }

    pub fn draw(&self, l: &Layout, c: Context, g: &mut G2d) {
        let border = Line::new([0.2, 0.2, 0.2, 0.5], 1.0);
        let fill = Polygon::new(type_color(&self.category));
        let mut edges: HashSet<Edge> = HashSet::new();
        for hex in self.cells.iter() {
            let candidates: HashSet<_> = hex.edges().iter().cloned().collect();
            edges = edges.symmetric_difference(&candidates).cloned().collect();
            fill.draw(&hex.vertices(l), &DEFAULT_DRAW_STATE, c.transform, g);
        }
        for edge in edges.iter() {
            border.draw(edge.vertices(l), &DEFAULT_DRAW_STATE, c.transform, g);
        }
    }
}
