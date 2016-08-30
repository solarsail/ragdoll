extern crate piston_window;

use piston_window::*;
use piston_window::character::CharacterCache;
use game::{GameContext, State, GameState};


static GAME_TITLE: &'static str = "Game Title";
static PROMPT: &'static str = "Press Any Key";

pub struct TitleState {
    next_state: Option<State>,
    show_prompt: bool,
    timer: f64,
}

impl TitleState {
    pub fn new() -> Self {
        TitleState {
            next_state: None,
            show_prompt: true,
            timer: 0.0,
        }
    }
}

impl GameState for TitleState {
    #[allow(unused_variables)]
    fn on_update(&mut self, gc: &mut GameContext, dt: f64/* in seconds */) {
        self.timer += dt;
        if self.timer > 1.0 {
            self.show_prompt = !self.show_prompt;
            self.timer = 0.0;
        }
    }

    fn on_render(&mut self, gc: &mut GameContext, e: &Event, w: &mut PistonWindow) {
        let center_x = gc.render_size[0] as f64 / 2.;
        let center_y = gc.render_size[1] as f64 / 2.;
        let title_text = Text::new_color([1.0; 4], 30);
        let prompt_text = Text::new_color([0.8, 0.8, 0.8, 1.0], 22);
        w.draw_2d(e, |c, g| {
            let font = gc.res.font();
            let title_width = font.width(30, GAME_TITLE);
            let prompt_width = font.width(22, PROMPT);
            clear([0.0, 0.0, 0.0, 1.0], g);
            title_text.draw(GAME_TITLE, font, &c.draw_state,
                c.transform.trans(center_x - title_width/2., center_y - 100.), g);
            if self.show_prompt {
                prompt_text.draw(PROMPT, font, &c.draw_state,
                    c.transform.trans(center_x - prompt_width/2., center_y + 30.), g);
            }
        });
    }

    #[allow(unused_variables)]
    fn on_input(&mut self, gc: &mut GameContext, input: Input) {
        match input {
            Input::Press(Button::Keyboard(_)) => {
                self.next_state = Some(State::Gameplay);
            }
            _ => {}
        }
    }

    fn state_changed(&self) -> Option<State> {
        self.next_state
    }
}