extern crate piston_window;

use piston_window::*;

use map::*;
use settings::*;
use game::GameState;
use game::states::*;
use resource::Resources;


#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum State {
    Opening,
    Title,
    Gameplay,
    Pause,
    Resume,
}

pub struct GameContext<'a> {
    pub cursor_screen_coord: [f64; 2],
    pub render_size: [u32; 2],
    pub scroll_rate: u32,
    pub res: &'a mut Resources,
}

pub struct Game<'a> {
    context: GameContext<'a>,
    window: &'a mut PistonWindow,
    states: Vec<Box<GameState>>,
    paused: bool,
    current_state: State,
}

impl<'a> Game<'a> {
    pub fn new(settings: Settings, window: &'a mut PistonWindow, res: &'a mut Resources) -> Self {
        Game {
            context: GameContext {
                render_size: [settings.window_width, settings.window_height],
                cursor_screen_coord: [0.0, 0.0],
                scroll_rate: settings.scroll_rate,
                res: res,
            },
            window: window,
            states: Vec::with_capacity(3),
            paused: false,
            current_state: State::Opening,
        }
    }

    fn make_context(&mut self, e: &Event) {
        e.mouse_cursor(|x, y| {
            self.context.cursor_screen_coord = [x, y];
        });
        e.render(|args| {
            self.context.render_size = [args.draw_width, args.draw_height];
        });
    }

    pub fn run(&mut self) {
        self.to_state(State::Opening);

        while let Some(e) = self.window.next() {
            let last = self.states.len() - 1;
            self.make_context(&e);
            match e {
                Event::Input(input) => {
                    let upmost = &mut self.states[last];
                    upmost.on_input(&mut self.context, input);
                }
                Event::Update(UpdateArgs { dt }) => {
                    {
                        let upmost = &mut self.states[last];
                        upmost.on_update(&mut self.context, dt);
                    }
                    let st = (&self.states[last]).state_changed();
                    match st {
                        Some(State::Pause) => {
                            self.pause();
                        }
                        Some(State::Resume) => {
                            self.resume();
                        }
                        Some(s) => {
                            self.to_state(s);
                        }
                        None => {}
                    }
                }
                Event::Render(_) => {
                    for s in self.states.iter_mut() {
                        s.on_render(&mut self.context, &e, self.window);
                    }
                }
                _ => {}
            }
        }

    }

    pub fn to_state(&mut self, s: State) {
        debug_assert!(!self.paused);
        let state: Box<GameState> = match s {
            State::Opening => Box::new(OpeningState::new(4.0, self.window)),
            _ => {
                let map = HexMap::new(5);
                let layout = Layout::new(POINTY_TOP, [20.0, 20.0], [200.0, 200.0]);
                Box::new(GamePlayState::new(map, layout))
            }
        };
        self.states.clear();
        self.states.push(state);
        self.current_state = s;
    }

    pub fn pause(&mut self) {
        //debug_assert!(self.current_state == State::Gameplay);
        let state = Box::new(PauseState::new());
        self.states.push(state);
        self.paused = true;
    }

    pub fn resume(&mut self) {
        debug_assert!(self.paused);
        self.states.pop();
        self.paused = false;
    }
}
