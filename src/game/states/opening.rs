extern crate piston_window;

use std::marker::PhantomData;
use piston_window::*;
use piston_window::rectangle::square;
use game::*;
use default;


pub struct OpeningState<'a> {
    total: f64,
    remaining: f64,
    image: Image,
    texture: G2dTexture<'a>,
    phantom: PhantomData<&'a i32>,
    done: bool,
}

impl<'a> OpeningState<'a> {
    pub fn new(t: f64, window: &mut PistonWindow) -> Self {
        let img_path = default::assets_path().join("images").join("rust-logo.png");
        OpeningState {
            total: t,
            remaining: t,
            image: Image::new().rect(square(0.0, 0.0, 200.0)),
            texture: Texture::from_path(
                &mut window.factory,
                &img_path,
                Flip::None,
                &TextureSettings::new()).unwrap(),
            phantom: PhantomData,
            done: false,
        }
    }

    fn mask_alpha(&self) -> f32 {
        let p = self.total / 5.0;
        if self.total - self.remaining < p {
            (1.0 - (self.total - self.remaining) / p) as f32
        } else if self.remaining < p {
            (1.0 - self.remaining / p) as f32
        } else {
            0.0
        }
    }
}

impl<'a> GameState for OpeningState<'a> {
    #[allow(unused_variables)]
    fn on_update(&mut self, gc: &mut GameContext, dt: f64) {
        self.remaining -= dt;
        if self.remaining < 0.0 {
            self.done = true;
        }
    }

    #[allow(unused_variables)]
    fn on_render(&mut self, gc: &mut GameContext, e: &Event, w: &mut PistonWindow) {
        let x = gc.render_size[0] / 2 - 100;
        let y = gc.render_size[1] / 2 - 100;
        w.draw_2d(e, |c, g| {
            clear([0.0; 4], g);
            self.image.draw(
                &self.texture,
                //default::draw_state(),
                &c.draw_state,
                c.transform.trans(x as f64, y as f64), g);
            //image(&self.texture, c.transform.trans(x as f64, y as f64), g);
            rectangle(
                [0.0, 0.0, 0.0, self.mask_alpha()],
                [0.0, 0.0, gc.render_size[0] as f64, gc.render_size[1] as f64],
                c.transform, g);
        });
    }

    #[allow(unused_variables)]
    fn on_input(&mut self, gc: &mut GameContext, input: Input) {
    }

    fn state_changed(&self) -> Option<State> {
        if self.done { Some(State::Title) } else { None }
    }
}