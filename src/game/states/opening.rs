use specs::{World, Entity, Gate};

use game::{InputHandler, State, Trans};
use resource::AssetManager;
use components::{Renderable, Position};


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
        let logo = Renderable::new("logo", 0, 100, 100);
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
        for mouse_btn in input_handler.clicked_iter() {
            debug!("mouse click: {:?}", mouse_btn);
        }
        Trans::None
    }
}
