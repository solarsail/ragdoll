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

    let mut window: PistonWindow =
        WindowSettings::new("Hello Piston!", [640, 480])
        .samples(4)
        .exit_on_esc(true)
        .build().unwrap();

    let map = HexMap::new(5);

    let mut game = Game::new(map, layout);
    /*
    let mut region1 = Region::new().category(Category::Player);
    region1.push(Hex::new(0, 0));
    region1.push(Hex::new(0, 1));
    region1.push(Hex::new(1, 0));

    let mut region2 = Region::new().category(Category::Neutral);
    region2.push(Hex::new(-2, 0));
    region2.push(Hex::new(-2, 1));
    region2.push(Hex::new(-2, -2));

    let mut region3 = Region::new().category(Category::Hostile);
    region3.push(Hex::new(3, 1));
    region3.push(Hex::new(2, 2));

    let mut region4 = Region::new().category(Category::Friendly);
    region4.push(Hex::new(-1, -3));
    region4.push(Hex::new(0, -3));
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
