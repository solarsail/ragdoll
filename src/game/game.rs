extern crate piston_window;
extern crate conrod;

use std::rc::Rc;

use piston_window::*;
use conrod::Ui;
use conrod::backend::piston_window as conrod_backend;

use map::*;
use settings::*;
use game::GameState;
use game::states::*;
use resource::Resources;
use default;


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
    pub ui: Rc<Ui>,
}

pub struct Game<'a> {
    context: GameContext<'a>,
    window: &'a mut PistonWindow,
    ui: Rc<Ui>,
    states: Vec<Box<GameState>>,
    paused: bool,
    current_state: State,
    text_texture_cache: conrod_backend::GlyphCache,
    image_map: conrod::image::Map<G2dTexture<'a>>,
}

impl<'a> Game<'a> {
    pub fn new(settings: Settings,
               window: &'a mut PistonWindow,
               res: &'a mut Resources) -> Self
    {
        let ui = Rc::new(conrod::UiBuilder::new().build());
        Game {
            context: GameContext {
                render_size: [settings.window_width, settings.window_height],
                cursor_screen_coord: [0.0, 0.0],
                scroll_rate: settings.scroll_rate,
                res: res,
                ui: ui.clone(),
            },
            window: window,
            ui: ui.clone(),
            states: Vec::with_capacity(3),
            paused: false,
            current_state: State::Opening,
            text_texture_cache: conrod_backend::GlyphCache::new(
                window, settings.window_width, settings.window_height),
            image_map: conrod::image::Map::new(),
        }
    }

    fn init_fonts(&mut self) {
        self.ui.fonts.insert_from_file(default::fonts_path().join("RussoOne-Regular.ttf"));
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
                    self.window.draw_2d(&e, |c, g| {
                        if let Some(primitives) = self.ui.draw_if_changed() {
                            fn texture_from_image<T>(img: &T) -> &T { img };
                            conrod_backend::draw(c, g, primitives,
                                &mut self.text_texture_cache,
                                &self.image_map,
                                texture_from_image);
                        }
                    });
                }
                _ => {}
            }
        }

    }

    pub fn to_state(&mut self, s: State) {
        debug_assert!(!self.paused);
        let state: Box<GameState> = match s {
            State::Opening => Box::new(OpeningState::new(4.0, self.window)),
            State::Title => Box::new(TitleState::new(&mut self.ui)),
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
