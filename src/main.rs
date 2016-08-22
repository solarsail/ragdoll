extern crate piston_window;

mod geometry;
mod map;
mod region;
mod hex;
mod game;

use piston_window::*;

use map::*;
use hex::*;
use region::{Region, Category};
use game::*;

fn main() {
    let layout = Layout::new(POINTY_TOP, [20.0, 20.0], [200.0, 200.0]);
    let wsize = [640, 480];

    let mut window: PistonWindow =
        WindowSettings::new("Hello Piston!", wsize)
        .samples(4)
        .exit_on_esc(true)
        .build().unwrap();

    let map = HexMap::new(5);

    let mut game = Game::new(wsize, map, layout);
    /*
    let mut region2 = Region::new(Category::Friendly);
    region2.push(Hex::new(-2, 0));
    region2.push(Hex::new(-2, 1));
    region2.push(Hex::new(-2, -2));
    */

    while let Some(e) = window.next() {
        match e {
            Event::Input(input) => {
                game.on_input(input);
            }
            Event::Render(_) => {
                game.on_render(&e, &mut window);
            }
            _ => {}
        }
    }
}
