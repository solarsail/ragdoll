use piston_window::*;

use settings::*;
use game::{GameState, StateMachine};
use game::states::*;
use resource::Resources;


pub struct GameContext<'a> {
    pub cursor_screen_coord: [f64; 2],
    pub render_size: [u32; 2],
    pub scroll_rate: u32,
    pub res: &'a mut Resources,
}

pub struct Game<'a> {
    context: GameContext<'a>,
    dfa: &'a mut StateMachine,
    window: &'a mut PistonWindow,
    states: &'a mut Vec<Box<GameState>>,
}

impl<'a> Game<'a> {
    pub fn new(settings: Settings,
               window: &'a mut PistonWindow,
               res: &'a mut Resources,
               dfa: &'a mut StateMachine,
               states: &'a mut Vec<Box<GameState>>) -> Self {
        Game {
            context: GameContext {
                render_size: [settings.window_width, settings.window_height],
                cursor_screen_coord: [0.0, 0.0],
                scroll_rate: settings.scroll_rate,
                res: res,
            },
            window: window,
            dfa: dfa,
            states: states,
        }
    }

    fn make_context(&mut self, e: &Input) {
        e.mouse_cursor(|x, y| {
            self.context.cursor_screen_coord = [x, y];
        });
        e.render(|args| {
            self.context.render_size = [args.draw_width, args.draw_height];
        });
    }

    pub fn run(&mut self) {
        while let Some(e) = self.window.next() {
            self.make_context(&e);
            match e {
                Input::Press(_) | Input::Release(_) | Input::Move(_) => {
                    let current = self.dfa.current_state_id();
                    self.states[current].on_input(&mut self.context, &mut self.dfa, &e);
                }
                Input::Update(UpdateArgs { dt }) => {
                    let current = self.dfa.current_state_id();
                    self.states[current].on_update(&mut self.context, &mut self.dfa, dt);
                }
                Input::Render(_) => {
                    for i in self.dfa.ui_stack() {
                        self.states[i].on_render(&mut self.context, &e, self.window);
                    }
                }
                _ => {}
            }
        }

    }

}
