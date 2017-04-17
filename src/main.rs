#[macro_use]
extern crate log;
extern crate env_logger;
extern crate sdl2;
extern crate hexgrid;
extern crate find_folder;
extern crate nalgebra as na;

mod default;
mod map;
mod game;
mod view;
mod settings;
mod resource;


use settings::*;
use game::Game;


fn main() {
    let settings = Settings::load("settings.ini");
    
    Game::start("random title", settings, |game| {
        game.run();
    });
}
