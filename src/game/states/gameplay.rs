extern crate piston_window;

use std::cell::Cell;

use piston_window::*;
use map::*;
use game::{GameContext, GameState, State};

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
    need_pause: Cell<bool>,
}

impl GamePlayState {
    pub fn new(map: HexMap, layout: Layout) -> GamePlayState {
        GamePlayState {
            map: map,
            layout: layout,
            cursor_region: Region::new(Category::Neutral),
            cursor_world_coord: [0.0, 0.0],
            origin: [0.0, 0.0],
            scroll: [Scroll::None, Scroll::None],
            need_pause: Cell::new(false),
        }
    }
}

impl GameState for GamePlayState {
    fn on_update(&mut self, gc: &mut GameContext, dt: f64/* in seconds */) {
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

    #[allow(unused_variables)]
    fn on_render(&mut self, gc: &mut GameContext, e: &Event, w: &mut PistonWindow) {
        w.draw_2d(e, |c, g| {
            clear([1.0; 4], g);
            let c = c.trans(-self.origin[0], -self.origin[1]);
            self.map.draw(&self.layout, c, g);
            self.cursor_region.draw(&self.layout, c, g);
        });
    }

    fn on_input(&mut self, gc: &mut GameContext, input: Input) {
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
                    Button::Keyboard(key) => {
                        match key {
                            Key::Up => {
                                self.scroll[1] = Scroll::Up;
                            }
                            Key::Down => {
                                self.scroll[1] = Scroll::Down;
                            }
                            Key::Left => {
                                self.scroll[0] = Scroll::Left;
                            }
                            Key::Right => {
                                self.scroll[0] = Scroll::Right;
                            }
                            _ => {}
                        }
                    }
                    _ => {}
                }
            }
            Input::Release(btn) => {
                match btn {
                    Button::Mouse(MouseButton::Left) => {
                        self.cursor_region.clear();
                    }
                    Button::Keyboard(key) => {
                        match key {
                            Key::Escape => {
                                self.need_pause.set(true);
                            }
                            Key::Up | Key::Down => {
                                self.scroll[1] = Scroll::None;
                            }
                            Key::Left | Key::Right => {
                                self.scroll[0] = Scroll::None;
                            }
                            _ => {}
                        }
                    }
                    _ => {}
                }
            }
            _ => {}
        }
    }

    fn state_changed(&self) -> Option<State> {
        if self.need_pause.get() {
            self.need_pause.set(false);
            Some(State::Pause)
        } else {
            None
        }
    }
}