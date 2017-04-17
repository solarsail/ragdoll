use sdl2::render::Renderer;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use game::{GameContext, StateTrans, GameState, StateMachine};


static GAME_TITLE: &'static str = "Game Title";
static PROMPT: &'static str = "Press Any Key";

pub struct TitleState {
    show_prompt: bool,
    timer: f64,
}

impl TitleState {
    pub fn new() -> Self {
        TitleState {
            show_prompt: true,
            timer: 0.0,
        }
    }
}

impl GameState for TitleState {
    #[allow(unused_variables)]
    fn on_update(&mut self, ctx: &mut GameContext, dfa: &mut StateMachine, dt: f64/* in seconds */) {
        self.timer += dt;
        if self.timer > 1.0 {
            self.show_prompt = !self.show_prompt;
            self.timer = 0.0;
        }
    }

    fn on_render(&mut self, ctx: &mut GameContext, r: &mut Renderer) {
        let center_x = ctx.render_size[0] / 2;
        let center_y = ctx.render_size[1] / 2;
        let title_surface = ctx.res.title_font().render(GAME_TITLE).blended(Color::RGBA(255, 87, 0, 255)).unwrap();
        let mut title_texture = r.create_texture_from_surface(&title_surface).unwrap();
        let prompt_surface = ctx.res.caption_font().render(PROMPT).blended(Color::RGBA(200, 200, 200, 255)).unwrap();
        let mut prompt_texture = r.create_texture_from_surface(&prompt_surface).unwrap();
        r.copy(&mut title_texture, None,
                Some(Rect::new(
                    (center_x - title_surface.width() / 2) as i32,
                    (center_y - 100) as i32,
                    title_surface.width(), title_surface.height()))).unwrap();
        if self.show_prompt {
            r.copy(&mut prompt_texture, None,
                Some(Rect::new(
                    (center_x - prompt_surface.width() / 2) as i32,
                    (center_y + 30) as i32,
                    prompt_surface.width(), prompt_surface.height()))).unwrap();
        }
    }

    fn on_input(&mut self, ctx: &mut GameContext, dfa: &mut StateMachine) {
        for _ in ctx.key_triggers.iter() {
            dfa.feed(StateTrans::Gameplay);
            break;
        }
    }
}