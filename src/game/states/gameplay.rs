use specs::{Planner, World, Entity, Gate, Join};
use sdl2::pixels::Color;

use game::{InputHandler, State, Trans};
use game::render::ScreenDimension;
use game::input::Keycode;
use resource::AssetManager;
use components::{Renderable, Position, Text, MainCamera, InputReceiver};
use systems::MovementSystem;


pub struct GameplayState {
    entities: Vec<Entity>,
}

impl GameplayState {
    pub fn new() -> GameplayState {
        GameplayState { entities: Vec::new() }
    }
}

impl State for GameplayState {
    fn on_start(&mut self, planner: &mut Planner<()>, _assets: &mut AssetManager) {
        planner.add_system(MovementSystem, "movement", 0);
        let mut world = planner.mut_world();
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

    fn on_stop(&mut self, planner: &mut Planner<()>, _assets: &mut AssetManager) {
        let mut ind: usize = 0;
        for (i, v) in planner.systems.iter().enumerate() {
            if v.name == "movement" {
                ind = i;
                break;
            }
        }
        planner.systems.remove(ind);
        let mut world = planner.mut_world();
        for e in &self.entities {
            world.delete_now(*e);
        }
    }

    fn update(&mut self, _planner: &mut Planner<()>, _assets: &mut AssetManager, dt: f32) -> Trans {
        Trans::None
    }
}
