extern crate piston_window;

use piston_window::*;
use piston_window::character::CharacterCache;
use game::{GameContext, StateTrans, GameState, StateMachine};

pub struct PauseState {
    text: Text,
}

impl PauseState {
    pub fn new() -> Self {
        PauseState {
            text: Text::new(22),
        }
    }
}

impl GameState for PauseState {
    #[allow(unused_variables)]
    fn on_update(&mut self, gc: &mut GameContext, dfa: &mut StateMachine, dt: f64/* in seconds */) {
    }

    fn on_render(&mut self, gc: &mut GameContext, e: &Input, w: &mut PistonWindow) {
        let center_x = gc.render_size[0] as f64 / 2.;
        let center_y = gc.render_size[1] as f64 / 2.;
        w.draw_2d(e, |c, g| {
            let font = gc.res.font();
            let str_width = font.width(22, "PAUSED");
            rectangle(
                [0.0, 0.0, 0.0, 0.4],
                [0.0, 0.0, gc.render_size[0] as f64, gc.render_size[1] as f64],
                c.transform, g);
            rectangle(
                [1.0, 1.0, 1.0, 0.8],
                [center_x, center_y, 200.0, 100.0],
                c.transform.trans(-100., -50.), g);
            self.text.draw("PAUSED",
                font,
                &c.draw_state,
                c.transform.trans(center_x - str_width/2., center_y), g);
        });
    }

    #[allow(unused_variables)]
    fn on_input(&mut self, gc: &mut GameContext, dfa: &mut StateMachine, input: &Input) {
        match *input {
            Input::Release(Button::Keyboard(Key::Escape)) => {
                dfa.feed(StateTrans::Resume);
            }
            _ => {}
        }
    }
}
