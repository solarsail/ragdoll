extern crate piston_window;

use piston_window::*;
use default;


pub struct Resources {
    font: Option<Glyphs>
}

impl Resources {
    pub fn new(window: &PistonWindow) -> Self {
        Resources {
            font: Glyphs::new(
                default::assets_path().join("fonts").join("RussoOne-Regular.ttf"),
                window.factory.clone()).ok()
        }
    }

    pub fn font(&mut self) -> &mut Glyphs {
        self.font.as_mut().unwrap()
    }
}