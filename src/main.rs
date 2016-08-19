extern crate piston_window;

mod geometry;
mod map;
//mod region;
mod hex;

use piston_window::*;
use map::*;
use hex::*;

fn main() {
    let layout = Layout::new(POINTY_TOP, [20.0, 20.0], [200.0, 200.0]);

    let mut window: PistonWindow =
        WindowSettings::new("Hello Piston!", [640, 480])
        .exit_on_esc(true).build().unwrap();

    let map = HexMap::new(5);

    while let Some(e) = window.next() {
        window.draw_2d(&e, |c, g| {
            clear([1.0; 4], g);
            map.draw(&layout, c, g);
        });
    }
}
