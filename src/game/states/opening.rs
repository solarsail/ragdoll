use specs::{World, Entity, Gate};

use game::{InputHandler, State, Trans};
use game::render::{RenderBuffer_1, RenderCommand};
use game::input::Click;
use resource::AssetManager;
use components::{Renderable, Position};
use def::{Point, Size};


pub struct OpeningState {
    logo: Option<Entity>,
}

impl OpeningState {
    pub fn new() -> OpeningState {
        OpeningState { logo: None }
    }
}
impl State for OpeningState {
    fn on_start(&mut self, world: &mut World, _assets: &mut AssetManager) {
        let logo = Renderable::new("logo", 100, 100);
        self.logo = Some(world
                             .create_now()
                             .with(logo)
                             .with(Position::new2(50, 50))
                             .build());
    }

    fn on_stop(&mut self, world: &mut World, _assets: &mut AssetManager) {
        world.delete_now(self.logo.unwrap());
    }

    fn update(&mut self, world: &mut World, _assets: &mut AssetManager, _dt: f32) -> Trans {
        let mut input_handler = world.write_resource::<InputHandler>().pass();
        /*
        let mut object_buffer = world.write_resource::<RenderBuffer_1>().pass();
        object_buffer.push_back(RenderCommand {
                                    texture_id: "NOT_FOUND".into(),
                                    pos: Point::new(50, 50),
                                    size: Size::new(100, 100),
                                    alpha: 128,
                                });
        */
        for click in input_handler.clicked_iter() {
            debug!("mouse click: {:?}", click);
        }
        Trans::None
    }
}
