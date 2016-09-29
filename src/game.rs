use amethyst::engine::{State, Trans};
use amethyst::context::Context;
use amethyst::processors::rendering::{Renderable, Camera, Projection};
use amethyst::ecs::{World, Join};

use processors::InputState;
use map::{Map, TileSettings, Surface, Terrain};
use settings::Settings;
use mesh;
use geometry;


pub struct Game;

impl State for Game {
    fn on_start(&mut self, context: &mut Context, world: &mut World) {
        let (w, h) = context.renderer.get_dimensions().unwrap();
        let aspect = w as f32 / h as f32;
        let eye = [0., -0.2, 1.5];
        let target = [0., 0., 0.];
        let up = [0., 1., 0.];

        /*
        // Get an Orthographic projection
        let projection = Projection::Orthographic {
            left: -1.0 * aspect,
            right: 1.0 * aspect,
            bottom: -1.0,
            top: 1.0,
            near: 0.0,
            far: 1.0,
        };
        */
        let projection = Projection::Perspective {
            fov: 60.0,
            aspect: aspect,
            near: 0.1,
            far: 100.,
        };

        // Add all resources
        let input_state = InputState::new();
        world.add_resource::<InputState>(input_state);
        world.add_resource::<Projection>(projection.clone());
        world.add_resource::<Settings>(Settings::new());

        let mut ts = TileSettings::new(0.15);
        ts.set_terrain_mesh(Terrain::Water, "hex".to_string());
        ts.set_terrain_mesh(Terrain::Basin, "hex".to_string());
        ts.set_terrain_mesh(Terrain::Plain, "hex".to_string());
        ts.set_terrain_mesh(Terrain::Hill, "hex".to_string());
        ts.set_terrain_mesh(Terrain::Plateau, "hex".to_string());
        ts.set_surface_texture(Surface::Soil, "white".to_string());
        ts.set_surface_texture(Surface::Grass, "green".to_string());
        ts.set_surface_texture(Surface::Forest, "green".to_string());
        ts.set_surface_texture(Surface::Sand, "red".to_string());
        ts.set_surface_texture(Surface::Snow, "white".to_string());
        ts.set_surface_texture(Surface::Ice, "blue".to_string());

        let map = Map::sample();

        // Create a camera entity
        let mut camera = Camera::new(projection, eye, target, up);
        camera.activate();
        world.create_now()
            .with(camera)
            .build();

        context.asset_manager.create_constant_texture("white", [1.0, 1.0, 1.0, 1.]);
        context.asset_manager.create_constant_texture("red", [1.0, 0.0, 0.0, 1.]);
        context.asset_manager.create_constant_texture("green", [0.0, 1.0, 0.0, 1.]);
        context.asset_manager.create_constant_texture("blue", [0.0, 0.0, 1.0, 1.]);
        // Generate a hex mesh
        context.asset_manager.load_mesh("hex", &mesh::simple_hex_mesh());

        for (coord, terrain, surface) in map.iter() {
            let (surface, terrain) = (ts.get_surface_texture(surface), ts.get_terrain_mesh(terrain));
            println!("{:?}, {:?}, {:?}", coord, terrain, surface);
            let mut tile = Renderable::new(terrain, surface, surface);
            tile.scale[0] = ts.radius();
            tile.scale[1] = ts.radius();
            tile.scale[2] = ts.radius();
            tile.translation[0] = (coord.q() as f32 + coord.r() as f32 * 0.5) * geometry::SQRT3 * ts.radius();
            tile.translation[1] = coord.r() as f32 * 1.5 * ts.radius();
            world.create_now()
                .with(tile)
                .with(coord)
                .build();
        }
    }

    fn update(&mut self, context: &mut Context, _: &mut World) -> Trans {
        // Exit if user hits Escape or closes the window
        use amethyst::context::event::{EngineEvent, Event, VirtualKeyCode};
        let engine_events = context.broadcaster.read::<EngineEvent>();
        for engine_event in engine_events.iter() {
            match engine_event.payload {
                Event::KeyboardInput(_, _, Some(VirtualKeyCode::Escape)) => return Trans::Quit,
                Event::Closed => return Trans::Quit,
                _ => (),
            }
        }

        Trans::None
    }
}