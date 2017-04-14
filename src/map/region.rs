use std::collections::HashSet;

use hexgrid::{Coordinates, EdgeCoordinates, Layout, PointPair};


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
    cells: HashSet<Coordinates>,
}

impl Region {
    pub fn new(c: Category) -> Region {
        Region {
            category: c,
            cells: HashSet::new()
        }
    }

    pub fn push(&mut self, c: Coordinates) {
        self.cells.insert(c);
    }

    pub fn clear(&mut self) {
        self.cells.clear();
    }

    pub fn draw(&self, l: &Layout, c: Context, g: &mut G2d) {
        let border = Line::new([0.2, 0.2, 0.2, 0.5], 1.0);
        let fill = Polygon::new(type_color(&self.category));
        let mut edges: HashSet<EdgeCoordinates> = HashSet::new();
        // 遍历包含的所有网格
        for hex in self.cells.iter() {
            // 计算轮廓
            let candidates: HashSet<EdgeCoordinates> = hex.adjacent_edges().into_iter().collect();
            edges = edges.symmetric_difference(&candidates).cloned().collect();
            // 绘制填充
            fill.draw(&l.vertices_of_hex(*hex).iter().map(|p| p.into()).collect::<Vec<[f64;2]>>(), &c.draw_state, c.transform, g);
        }
        // 绘制轮廓
        for edge in edges.iter() {
            border.draw(l.vertices_of_edge(*edge), &c.draw_state, c.transform, g);
        }
    }
}
