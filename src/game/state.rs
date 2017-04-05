extern crate piston_window;

use piston_window::*;
use game::{State, GameContext};


pub trait GameState {
    fn on_update(&mut self, gc: &mut GameContext, dt: f64);
    fn on_input(&mut self, gc: &mut GameContext, input: &Input);
    fn on_render(&mut self, gc: &mut GameContext, e: &Input, w: &mut PistonWindow);
    fn state_changed(&self) -> Option<State>;
}
