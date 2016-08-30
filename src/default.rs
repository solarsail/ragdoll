extern crate piston_window;
extern crate find_folder;

use std::path::PathBuf;
use piston_window::draw_state::*;


static DEFAULT_DRAW_STATE: DrawState = DrawState {
    scissor: None,
    stencil: None,
    blend: Some(Blend::Alpha)
};


pub fn draw_state<'a>() -> &'a DrawState {
    &DEFAULT_DRAW_STATE
}

pub fn assets_path() -> PathBuf {
    find_folder::Search::ParentsThenKids(3, 3).for_folder("assets").unwrap()
}