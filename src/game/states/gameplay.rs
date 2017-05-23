use specs::{World, Entity, Gate, Join};
use sdl2::pixels::Color;

use game::{InputHandler, State, Trans};
use game::render::ScreenDimension;
use game::input::Keycode;
use resource::AssetManager;
use components::{Renderable, Position, Text, MainCamera, InputReceiver};


pub struct GameplayState {
    entities: Vec<Entity>,
}

impl GameplayState {
    pub fn new() -> GameplayState {
        GameplayState { entities: Vec::new() }
    }
}

impl State for GameplayState {
    fn on_start(&mut self, world: &mut World, _assets: &mut AssetManager) {
        let screen_dim = world.read_resource::<ScreenDimension>().pass();
        let rect = Renderable::new("rect", 100, 100);
        let player = Renderable::new("player", 32, 32);
        self.entities
            .push(world
                      .create_now()
                      .with(rect)
                      .with(Position::new2(0, 0))
                      .build());
        self.entities
            .push(world
                      .create_now()
                      .with(player)
                      .with(Position::new2(100, 100))
                      .with(MainCamera::new(screen_dim.w, screen_dim.h))
                      .with(InputReceiver)
                      .build());
    }

    fn on_stop(&mut self, world: &mut World, _assets: &mut AssetManager) {
        for e in &self.entities {
            world.delete_now(*e);
        }
    }

    fn update(&mut self, world: &mut World, _assets: &mut AssetManager, dt: f32) -> Trans {
        let mut input_handler = world.write_resource::<InputHandler>().pass();
        let (ps, rs) = (world.write::<Position>(), world.read::<InputReceiver>());
        // TODO: use unit vector and constant speed
        for (mut p, r) in (&mut ps.pass(), &rs.pass()).join() {
            if input_handler.key_down(Keycode::W) {
                p.y -= (100.0 * dt) as i32;
            } else if input_handler.key_down(Keycode::S) {
                p.y += (100.0 * dt) as i32;
            }
            if input_handler.key_down(Keycode::A) {
                p.x -= (100.0 * dt) as i32;
            } else if input_handler.key_down(Keycode::D) {
                p.x += (100.0 * dt) as i32;
            }
        }
        for click in input_handler.clicked_iter() {
            debug!("mouse click: {:?}", click);
        }
        Trans::None
    }
}
