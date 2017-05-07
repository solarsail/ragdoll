use sdl2::render::Renderer;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use game::{GameContext, GameState, StateTrans, StateMachine};
use default;


pub struct OpeningState {
    total: f64,
    remaining: f64,
    logo_width: u32,
    logo_height: u32,
}

impl OpeningState {
    pub fn new(t: f64, logo_width: u32, logo_height: u32) -> Self {
        OpeningState {
            total: t,
            remaining: t,
            logo_width: logo_width,
            logo_height: logo_height,
        }
    }

    fn mask_alpha(&self) -> u8 {
        let p = self.total / 4.0;
        if self.total - self.remaining < p {
            ((1.0 - (self.total - self.remaining) / p) * 255.0) as u8
        } else if self.remaining < p {
            ((1.0 - self.remaining / p) * 255.0) as u8
        } else {
            0
        }
    }
}

impl GameState for OpeningState {
    #[allow(unused_variables)]
    fn on_update(&mut self, ctx: &mut GameContext, dfa: &mut StateMachine, dt: f64) {
        self.remaining -= dt;
        if self.remaining < 0.0 {
            dfa.feed(StateTrans::Title);
        }
    }

    fn on_render(&mut self, ctx: &mut GameContext, r: &mut Renderer) {
        let x = (ctx.render_size[0] - self.logo_width) / 2;
        let y = (ctx.render_size[1] - self.logo_height) / 2;
        let rect = Rect::new(x as i32, y as i32, self.logo_width, self.logo_height);
        r.copy(ctx.res.logo_texture(), None, Some(rect)).unwrap();
        r.set_draw_color(Color::RGBA(0, 0, 0, self.mask_alpha()));
        r.fill_rect(Some(rect));
    }

    fn on_input(&mut self, ctx: &mut GameContext, dfa: &mut StateMachine) {
        for key in ctx.key_triggers.iter() {
            if *key == Keycode::Escape {
                // TODO: 使用自定义类型解耦？
                dfa.feed(StateTrans::Title);
                break;
            }
        }
    }
}
