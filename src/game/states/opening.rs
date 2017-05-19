use specs::{World, Entity, Gate, Join};

use game::{InputHandler, State, Trans};
use game::render::{RenderBuffer_1, RenderCommand};
use game::input::Click;
use resource::AssetManager;
use components::{Renderable, Position};
use def::{Point, Size};


pub struct OpeningState {
    logo: Option<Entity>,
    total: f32,
    remaining: f32,
}

impl OpeningState {
    pub fn new(elapse: f32) -> OpeningState {
        OpeningState {
            logo: None,
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
    fn on_start(&mut self, world: &mut World, _assets: &mut AssetManager) {
        let logo = Renderable::new("logo1", 100, 100);
        self.logo = Some(world
                             .create_now()
                             .with(logo)
                             .with(Position::new2(50, 50))
                             .build());
    }

    fn on_stop(&mut self, world: &mut World, _assets: &mut AssetManager) {
        world.delete_now(self.logo.unwrap());
    }

    fn update(&mut self, world: &mut World, _assets: &mut AssetManager, dt: f32) -> Trans {
        self.remaining -= dt;
        if self.remaining < 0.0 {
            return Trans::None; // TODO: trans
        }
        let mut input_handler = world.write_resource::<InputHandler>().pass();
        let (entities, mut renderables) = (world.entities(), world.write::<Renderable>());
        for (entity, mut r) in (&entities.pass(), &mut renderables.pass()).join() {
            if entity == self.logo.unwrap() {
                r.alpha = self.logo_alpha();
            }
        }

        for click in input_handler.clicked_iter() {
            debug!("mouse click: {:?}", click);
        }
        Trans::None
    }
}
