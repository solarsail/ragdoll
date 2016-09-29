use std::sync::{Mutex, Arc};

use amethyst::context::Context;
use amethyst::ecs::{World, Join, VecStorage, Component, Processor, RunArg};
use amethyst::processors::rendering::{RenderingProcessor, Renderable, Light, Camera, Projection};

use tile::TileSettings;
use geometry::SQRT3;
use map::Map;


pub struct MapProcessor {
    init: bool,
}

unsafe impl Sync for MapProcessor {  }

impl MapProcessor {
    pub fn new() -> Self {
        MapProcessor {
            init: false,
        }
    }
}

impl Processor<Arc<Mutex<Context>>> for MapProcessor {
    fn run(&mut self, arg: RunArg, context: Arc<Mutex<Context>>) {
        if self.init { return; }

        let (map, ts, mut renderables) = arg.fetch(|w| (
            w.read_resource::<Map>(),
            w.read_resource::<TileSettings>(),
            w.write::<Renderable>()));

        for (coord, terrain, surface) in map.iter() {
            let (surface, terrain) = (ts.get_surface_texture(surface), ts.get_terrain_mesh(terrain));
            let mut tile = Renderable::new(terrain, surface, surface);
            for i in 0..2 {
                tile.scale[i] = ts.radius();
            }
            tile.translation[0] = (coord.q() as f32 + coord.r() as f32 * 0.5) * SQRT3 * ts.radius();
            tile.translation[1] = coord.r() as f32 * 1.5 * ts.radius();
            let map_entity = arg.create();
            renderables.insert(map_entity, tile);
        }
        self.init = true;
    }

}
