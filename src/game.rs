extern crate piston_window;

use piston_window::*;

use hex::*;
use region::*;
use map::*;


#[derive(Debug)]
enum State {
    Opening,
    Title,
    Gameplay,
    Pause,
}

enum Scroll {
    None, Left, Right, Up, Down
}

pub struct Game {
    state: State,
    windows_size: [u32; 2],
    map: HexMap,
    layout: Layout,
    cursor_region: Region,
    cursor_coord: [f64; 2],
    scroll: Scroll
}

impl Game {
    pub fn new(size: [u32; 2], map: HexMap, layout: Layout) -> Self {
        Game {
            state: State::Gameplay,
            windows_size: size,
            map: map,
            layout: layout,
            cursor_region: Region::new(Category::Neutral),
            cursor_coord: [0.0, 0.0],
            scroll: Scroll::None
        }
    }

    pub fn on_render(&self, e: &Event, w: &mut PistonWindow) {
        w.draw_2d(e, |c, g| {
            clear([1.0; 4], g);
            self.map.draw(&self.layout, c, g);
            self.cursor_region.draw(&self.layout, c, g);
        });
    }

    pub fn on_input(&mut self, input: Input) {
        match self.state {
            State::Gameplay => self.on_gameplay_input(input),
            _ => {}
        }
    }

    fn on_gameplay_input(&mut self, input: Input) {
        match input {
            Input::Move(m) => {
                match m {
                    Motion::MouseCursor(x, y) => {
                        self.cursor_coord = [x, y];
                        if x < 20.0 {
                            self.scroll = Scroll::Left; 
                        } else if x > self.windows_size[0] as f64 - 20.0 {
                            self.scroll = Scroll::Right;
                        } else {
                            self.scroll = Scroll::None;
                        }
                    }
                    _ => {}
                }
            }
            Input::Press(btn) => {
                match btn {
                    Button::Mouse(MouseButton::Left) => {
                        let hex = Hex::from_pixel(self.cursor_coord, &self.layout);
                        self.cursor_region.push(hex);
                    }
                    _ => {}
                }
            }
            Input::Release(btn) => {
                match btn {
                    Button::Mouse(MouseButton::Left) => {
                        self.cursor_region.clear();
                    }
                    _ => {}
                }
            }
            _ => {}
        }
    }
}