extern crate piston_window;

use piston_window::*;

use hex::*;
use map::*;
use settings::*;
use game::GameState;
use game::states::*;


#[derive(Debug)]
pub enum State {
    Opening,
    Title,
    Gameplay,
    Pause,
}

pub struct GameContext {
    pub cursor_screen_coord: [f64; 2],
    pub render_size: [u32; 2],
    pub scroll_rate: u32
}

pub struct Game<'a> {
    states: Vec<Box<GameState>>,
    context: GameContext,
    window: &'a mut PistonWindow,
}

impl<'a> Game<'a> {
    pub fn new(settings: Settings, window: &'a mut PistonWindow) -> Self {
        Game {
            states: Vec::with_capacity(3),
            context: GameContext {
                render_size: [settings.window_width, settings.window_height],
                cursor_screen_coord: [0.0, 0.0],
                scroll_rate: settings.scroll_rate,
            },
            window: window
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
        let last = self.states.len() - 1;

        while let Some(e) = self.window.next() {
            self.make_context(&e);
            match e {
                Event::Input(input) => {
                    let upmost = &mut self.states[last];
                    upmost.on_input(&self.context, input);
                }
                Event::Update(UpdateArgs { dt }) => {
                    let upmost = &mut self.states[last];
                    upmost.on_update(&self.context, dt);
                }
                Event::Render(_) => {
                    for s in self.states.iter_mut() {
                        s.on_render(&self.context, &e, self.window);
                    }
                }
                _ => {}
            }
        }

    }

    pub fn push_state(&mut self, s: State) {
        let state: Box<GameState> = match s {
            State::Gameplay => {
                let map = HexMap::new(5);
                let layout = Layout::new(POINTY_TOP, [20.0, 20.0], [200.0, 200.0]);
                Box::new(GamePlayState::new(map, layout))
            }
            _ => Box::new(OpeningState::new(2.0, self.window))
        };
        self.states.push(state);
    }
}
