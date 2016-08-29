extern crate piston_window;

use piston_window::*;
use game::GameContext;

pub trait GameState {
    fn on_update(&mut self, gc: &GameContext, dt: f64);
    fn on_input(&mut self, gc: &GameContext, input: Input);
    fn on_render(&mut self, gc: &GameContext, e: &Event, w: &mut PistonWindow);
}