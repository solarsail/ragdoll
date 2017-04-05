extern crate piston_window;

use std::cell::Cell;

use piston_window::*;
use map::*;
use game::{GameContext, GameState, State};
use view::View;
use geometry::*;

#[derive(PartialEq, Eq)]
enum Scroll {
    None, Left, Right, Up, Down
}

const SCROLL_AREA: f64 = 5.0;


pub struct GamePlayState {
    map: HexMap,
    layout: Layout,
    cursor_region: Region,
    scroll: [Scroll; 2],
    mouse_scroll_lock: bool,
    need_pause: Cell<bool>,
    map_view: View,
    ui_view: View,
}

impl GamePlayState {
    pub fn new(map: HexMap, layout: Layout) -> GamePlayState {
        GamePlayState {
            map: map,
            layout: layout,
            cursor_region: Region::new(Category::Neutral),
            scroll: [Scroll::None, Scroll::None],
            mouse_scroll_lock: false,
            need_pause: Cell::new(false),
            map_view: View::new(),
            ui_view: View::new(),
        }
    }
}

impl GameState for GamePlayState {
    fn on_update(&mut self, gc: &mut GameContext, dt: f64/* in seconds */) {
        self.map_view.set_size(gc.render_size[0] as f64, gc.render_size[1] as f64);
        let ds = gc.scroll_rate as f64 * dt;
        match self.scroll[0] {
            Scroll::Left => {
                self.map_view.trans_self(-ds, 0.0);
            }
            Scroll::Right => {
                self.map_view.trans_self(ds, 0.0);
            }
            _ => {}
        }
        match self.scroll[1] {
            Scroll::Up=> {
                self.map_view.trans_self(0.0, -ds);
            }
            Scroll::Down=> {
                self.map_view.trans_self(0.0, ds);
            }
            _ => {}
        }
    }

    #[allow(unused_variables)]
    fn on_render(&mut self, gc: &mut GameContext, e: &Input, w: &mut PistonWindow) {
        w.draw_2d(e, |c, g| {
            clear([1.0; 4], g);
            let c = c.append_transform(self.map_view.w2s_trans);
            // TODO: culling: use view or draw_state.scissor? how to use it?
            self.map.draw(&self.layout, &self.map_view, c, g);
            self.cursor_region.draw(&self.layout, c, g);
        });
    }

    fn on_input(&mut self, gc: &mut GameContext, input: &Input) {
        match *input {
            Input::Move(m) => {
                match m {
                    Motion::MouseCursor(x, y) if !self.mouse_scroll_lock => {
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
                        let cursor_world_coord = transform_pos(self.map_view.s2w_trans, gc.cursor_screen_coord);
                        let hex = Hex::from_pixel(cursor_world_coord, &self.layout);
                        self.cursor_region.push(hex);
                    }
                    Button::Keyboard(key) => {
                        match key {
                            Key::Up | Key::Down | Key::Left | Key::Right => {
                                self.mouse_scroll_lock = true;
                            }
                            _ => {}
                        }
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
                                if self.scroll[0] == Scroll::None {
                                    self.mouse_scroll_lock = false;
                                }
                            }
                            Key::Left | Key::Right => {
                                self.scroll[0] = Scroll::None;
                                if self.scroll[1] == Scroll::None {
                                    self.mouse_scroll_lock = false;
                                }
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