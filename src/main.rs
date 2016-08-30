extern crate piston_window;

mod default;
mod map;
mod game;
mod settings;
mod resource;

use piston_window::*;

use game::*;
use settings::*;

fn main() {
    let settings = Settings::load("settings.ini");
    let wsize = [settings.window_width, settings.window_height];

    let mut window: PistonWindow =
        WindowSettings::new("Hello Piston!", wsize)
        .samples(4)
        .exit_on_esc(false)
        .build().unwrap();

    let mut res = resource::Resources::new(&window);

    let mut game = Game::new(settings, &mut window, &mut res);

    game.run();
}
