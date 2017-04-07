extern crate piston_window;

use piston_window::*;
use piston_window::rectangle::rectangle_by_corners;
use game::{GameContext, GameState, StateTrans, StateMachine};
use default;


pub struct OpeningState {
    total: f64,
    remaining: f64,
    image: Image,
    logo_width: u32,
    logo_height: u32,
}

impl OpeningState {
    pub fn new(t: f64, logo_width: u32, logo_height: u32) -> Self {
        OpeningState {
            total: t,
            remaining: t,
            image: Image::new().rect(rectangle_by_corners(0.0, 0.0, logo_width as f64, logo_height as f64)),
            logo_width: logo_width,
            logo_height: logo_height,
        }
    }

    fn mask_alpha(&self) -> f32 {
        let p = self.total / 4.0;
        if self.total - self.remaining < p {
            (1.0 - (self.total - self.remaining) / p) as f32
        } else if self.remaining < p {
            (1.0 - self.remaining / p) as f32
        } else {
            0.0
        }
    }
}

impl GameState for OpeningState {
    #[allow(unused_variables)]
    fn on_update(&mut self, gc: &mut GameContext, dfa: &mut StateMachine, dt: f64) {
        self.remaining -= dt;
        if self.remaining < 0.0 {
            dfa.feed(StateTrans::Title);
        }
    }

    fn on_render(&mut self, gc: &mut GameContext, e: &Input, w: &mut PistonWindow) {
        let x = (gc.render_size[0] - self.logo_width) / 2;
        let y = (gc.render_size[1] - self.logo_height) / 2;
        w.draw_2d(e, |c, g| {
            clear([0.0; 4], g);
            self.image.draw(
                gc.res.logo_texture(),
                &c.draw_state,
                c.transform.trans(x as f64, y as f64), g);
            rectangle(
                [0.0, 0.0, 0.0, self.mask_alpha()],
                [0.0, 0.0, gc.render_size[0] as f64, gc.render_size[1] as f64],
                c.transform, g);
        });
    }

    #[allow(unused_variables)]
    fn on_input(&mut self, gc: &mut GameContext, dfa: &mut StateMachine, input: &Input) {
        match *input {
            Input::Release(Button::Keyboard(key)) => {
                match key {
                    Key::Escape => {
                        dfa.feed(StateTrans::Title);
                    }
                    _ => {}
                }
            }
            _ => {}
        }
    }
}