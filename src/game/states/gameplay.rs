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
                      .with(Position::new2(0.0, 0.0))
                      .build());
        self.entities
            .push(world
                      .create_now()
                      .with(player)
                      .with(Position::new2(100.0, 100.0))
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

        for click in input_handler.clicked_iter() {
            debug!("mouse click: {:?}", click);
        }
        Trans::None
    }
}
