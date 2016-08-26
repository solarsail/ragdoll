extern crate piston_window;

use piston_window::*;

use hex::*;
use region::*;
use map::*;
use settings::*;


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

const SCROLL_AREA :f64 = 5.0;

pub struct Game {
    state: State,
    render_size: [u32; 2],
    map: HexMap,
    layout: Layout,
    cursor_region: Region,
    cursor_coord: [f64; 2],
    origin: [f64; 2],
    scroll: [Scroll; 2],
    scroll_rate: u32
}

impl Game {
    pub fn new(settings: Settings, map: HexMap, layout: Layout) -> Self {
        Game {
            state: State::Gameplay,
            render_size: [settings.window_width, settings.window_height],
            map: map,
            layout: layout,
            cursor_region: Region::new(Category::Neutral),
            cursor_coord: [0.0, 0.0],
            origin: [0.0, 0.0],
            scroll: [Scroll::None, Scroll::None],
            scroll_rate: settings.scroll_rate,
        }
    }

    pub fn on_update(&mut self, dt: f64/* in seconds */) {
        let ds = self.scroll_rate as f64 * dt;
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

    pub fn on_render(&mut self, e: &Event, w: &mut PistonWindow) {
        if let Some(args) = e.render_args() {
            self.render_size = [args.draw_width, args.draw_height];
        }
        w.draw_2d(e, |c, g| {
            clear([1.0; 4], g);
            let c = c.trans(-self.origin[0], -self.origin[1]);
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
                        if x < SCROLL_AREA {
                            self.scroll[0] = Scroll::Left;
                        } else if x > self.render_size[0] as f64 - SCROLL_AREA {
                            self.scroll[0] = Scroll::Right;
                        } else {
                            self.scroll[0] = Scroll::None;
                        }
                        if y < SCROLL_AREA {
                            self.scroll[1] = Scroll::Up;
                        } else if y > self.render_size[1] as f64 - SCROLL_AREA {
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