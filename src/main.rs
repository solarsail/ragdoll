extern crate piston_window;

mod default;
mod geometry;
mod map;
mod region;
mod hex;
mod game;
mod settings;

use piston_window::*;

use game::*;
use settings::*;

fn main() {
    let settings = Settings::load("settings.ini");
    let wsize = [settings.window_width, settings.window_height];

    let mut window: PistonWindow =
        WindowSettings::new("Hello Piston!", wsize)
        .samples(4)
        .exit_on_esc(true)
        .build().unwrap();

    let mut game = Game::new(settings, &mut window);

    game.push_state(State::Gameplay);
    game.run();
}
