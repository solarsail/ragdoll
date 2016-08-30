extern crate piston_window;

use piston_window::draw_state::*;


static DEFAULT_DRAW_STATE: DrawState = DrawState {
    scissor: None,
    stencil: None,
    blend: Some(Blend::Alpha)
};


pub fn draw_state<'a>() -> &'a DrawState {
    &DEFAULT_DRAW_STATE
}