use specs::{Planner, Entity, Gate, Join};
use sdl2::pixels::Color;

use game::{InputHandler, State, Trans};
use game::input::Keycode;
use game::render::ScreenDimension;
use game::states::GameplayState;
use resource::AssetManager;
use components::{Renderable, Position, Text};


pub struct OpeningState {
    entities: Vec<Entity>,
    total: f32,
    remaining: f32,
}

impl OpeningState {
    pub fn new(elapse: f32) -> OpeningState {
        OpeningState {
            entities: Vec::new(),
            total: elapse,
            remaining: elapse,
        }
    }

    fn logo_alpha(&self) -> u8 {
        let p = self.total / 4.0;
        if self.total - self.remaining < p {
            ((self.total - self.remaining) / p * 255.0) as u8
        } else if self.remaining < p {
            (self.remaining / p * 255.0) as u8
        } else {
            255
        }
    }
}

impl State for OpeningState {
    fn on_start(&mut self, planner: &mut Planner<()>, _assets: &mut AssetManager) {
        let mut world = planner.mut_world();
        let screen_dim = world.read_resource::<ScreenDimension>().pass();
        let p = Position::new2((screen_dim.w as f32 - 200.0) / 2.0,
                               (screen_dim.h as f32 - 200.0) / 2.0);
        let logo = Renderable::new("logo", 200, 200);
        let text = Text::new("content",
                             "this is a test 测试",
                             Color::RGBA(200, 200, 200, 200),
                             100);
        self.entities
            .push(world.create_now().with(logo).with(p).build());
        self.entities
            .push(world
                      .create_now()
                      .with(text)
                      .with(Position::new2(100.0, 100.0))
                      .build());
    }

    fn on_stop(&mut self, planner: &mut Planner<()>, _assets: &mut AssetManager) {
        for e in &self.entities {
            planner.mut_world().delete_now(*e);
        }
    }

    fn update(&mut self, planner: &mut Planner<()>, _assets: &mut AssetManager, dt: f32) -> Trans {
        self.remaining -= dt;
        let mut done = false;
        if self.remaining < 0.0 {
            done = true;
        } else {
            let mut world = planner.mut_world();
            let mut input_handler = world.write_resource::<InputHandler>().pass();
            let (entities, renderables) = (world.entities(), world.write::<Renderable>());
            for (entity, mut r) in (&entities.pass(), &mut renderables.pass()).join() {
                r.alpha = self.logo_alpha();
            }

            if input_handler.key_down(Keycode::Escape) {
                done = true;
            }
        }
        if done {
            Trans::Switch(Box::new(GameplayState::new()))
        } else {
            Trans::None
        }
    }
}
