#[macro_use]
extern crate log;
extern crate log_panics;
extern crate log4rs;
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
    log4rs::init_file("config/log4rs.yaml", Default::default()).unwrap();
    log_panics::init();

    let settings = Settings::load("config/settings.ini");
    
    Game::start("random title", settings, |game| {
        game.run();
    });
}
