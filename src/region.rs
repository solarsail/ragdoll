extern crate piston_window;

use std::collections::HashSet;

use piston_window::{Context, G2d};
use piston_window::polygon::*;
use piston_window::line::*;

use hex::*;
use default;


pub enum Category {
    Neutral,
    Friendly,
    Hostile,
    Player,
}

fn type_color(category: &Category) -> [f32; 4] {
    match category {
        &Category::Neutral => [0.9, 0.9, 0.0, 0.5],
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
    pub fn new(c: Category) -> Region {
        Region {
            category: c,
            cells: HashSet::new()
        }
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
        // 遍历包含的所有网格
        for hex in self.cells.iter() {
            // 计算轮廓
            let candidates: HashSet<_> = hex.edges().iter().cloned().collect();
            edges = edges.symmetric_difference(&candidates).cloned().collect();
            // 绘制填充
            fill.draw(&hex.vertices(l), default::draw_state(), c.transform, g);
        }
        // 绘制轮廓
        for edge in edges.iter() {
            border.draw(edge.vertices(l), default::draw_state(), c.transform, g);
        }
    }
}
