extern crate piston_window;

use std::path::Path;
use piston_window::*;
use piston_window::rectangle::square;
use game::{GameContext, GameState};
use default;


pub struct OpeningState<'a> {
    elapse: f64,
    texture: G2dTexture<'a>
}

impl<'a> OpeningState<'a> {
    pub fn new(t: f64, window: &mut PistonWindow) -> Self {
        OpeningState {
            elapse: t,
            texture: Texture::from_path(
                &mut window.factory,
                Path::new("assets/images/rust-logo.png"),
                Flip::None,
                &TextureSettings::new()).unwrap()
        }
    }
}

impl<'a> GameState for OpeningState<'a> {
    fn on_update(&mut self, gc: &GameContext, dt: f64) {
        self.elapse -= dt;
        if self.elapse < 0.0 {
            // go to gameplay state
        }
    }

    fn on_render(&mut self, gc: &GameContext, e: &Event, w: &mut PistonWindow) {
        let x = gc.render_size[0] / 2;
        let y = gc.render_size[1] / 2;
        w.draw_2d(e, |c, g| {
            clear([0.0; 4], g);
            image(&self.texture, c.transform.trans(x as f64, y as f64), g);
        });
    }

    fn on_input(&mut self, gc: &GameContext, input: Input) {
    }
}