extern crate piston_window;

use piston_window::*;
use hex::*;
use map::*;
use region::{Region, Category};
use game::GameContext;
use gamestate::*;

enum Scroll {
    None, Left, Right, Up, Down
}

const SCROLL_AREA: f64 = 5.0;

pub struct GamePlayState {
    map: HexMap,
    layout: Layout,
    cursor_region: Region,
    cursor_world_coord: [f64; 2],
    origin: [f64; 2],
    scroll: [Scroll; 2],
}

impl GamePlayState {
    pub fn new(map: HexMap, layout: Layout) -> GamePlayState {
        GamePlayState {
            map: map,
            layout: layout,
            cursor_region: Region::new(Category::Neutral),
            cursor_world_coord: [0.0, 0.0],
            origin: [0.0, 0.0],
            scroll: [Scroll::None, Scroll::None]
        }
    }
}

impl GameState for GamePlayState {
    fn on_update(&mut self, gc: &GameContext, dt: f64/* in seconds */) {
        let ds = gc.scroll_rate as f64 * dt;
        match self.scroll[0] {
            Scroll::Left => {
                self.origin[0] -= ds;
            }
            Scroll::Right => {
                self.origin[0] += ds;
            }
            _ => {}
        }
        match self.scroll[1] {
            Scroll::Up=> {
                self.origin[1] -= ds;
            }
            Scroll::Down=> {
                self.origin[1] += ds;
            }
            _ => {}
        }
    }

    fn on_render(&mut self, gc: &GameContext, e: &Event, w: &mut PistonWindow) {
        w.draw_2d(e, |c, g| {
            clear([1.0; 4], g);
            let c = c.trans(-self.origin[0], -self.origin[1]);
            self.map.draw(&self.layout, c, g);
            self.cursor_region.draw(&self.layout, c, g);
        });
    }

    fn on_input(&mut self, gc: &GameContext, input: Input) {
        match input {
            Input::Move(m) => {
                match m {
                    Motion::MouseCursor(x, y) => {
                        self.cursor_world_coord = [x+self.origin[0], y+self.origin[1]];
                        if x < SCROLL_AREA {
                            self.scroll[0] = Scroll::Left;
                        } else if x > gc.render_size[0] as f64 - SCROLL_AREA {
                            self.scroll[0] = Scroll::Right;
                        } else {
                            self.scroll[0] = Scroll::None;
                        }
                        if y < SCROLL_AREA && y > 0.0 {
                            self.scroll[1] = Scroll::Up;
                        } else if y > gc.render_size[1] as f64 - SCROLL_AREA {
                            self.scroll[1] = Scroll::Down;
                        } else {
                            self.scroll[1] = Scroll::None;
                        }
                    }
                    _ => {}
                }
            }
            Input::Press(btn) => {
                match btn {
                    Button::Mouse(MouseButton::Left) => {
                        let hex = Hex::from_pixel(self.cursor_world_coord, &self.layout);
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