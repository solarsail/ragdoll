use sdl2::render::Renderer;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use game::{GameContext, StateTrans, GameState, StateMachine};

pub struct PauseState {
    //text: Text,
}

impl PauseState {
    pub fn new() -> Self {
        PauseState {
            //text: Text::new(22),
        }
    }
}

impl GameState for PauseState {
    #[allow(unused_variables)]
    fn on_update(&mut self, ctx: &mut GameContext, dfa: &mut StateMachine, dt: f64/* in seconds */) {
    }

    fn on_render(&mut self, ctx: &mut GameContext, r: &mut Renderer) {
        let center_x = ctx.render_size[0] / 2;
        let center_y = ctx.render_size[1] / 2;
        let surface = ctx.res.caption_font().render("PAUSED").blended(Color::RGBA(255, 87, 0, 255)).unwrap();
        let mut texture = r.create_texture_from_surface(&surface).unwrap();

        r.set_draw_color(Color::RGBA(0, 0, 0, 102));
        r.fill_rect(None).unwrap();
        r.set_draw_color(Color::RGBA(255, 255, 255, 204));
        r.fill_rect(Some(Rect::new((center_x - 100) as i32, (center_y - 50) as i32, 200, 100))).unwrap();
        r.copy(&mut texture, None, Some(Rect::new((center_x - surface.width() / 2) as i32,
                                        (center_y - surface.height() / 2) as i32, surface.width(), surface.height()))).unwrap();
    }

    fn on_input(&mut self, ctx: &mut GameContext, dfa: &mut StateMachine) {
        for key in ctx.key_triggers.iter() {
            if *key == Keycode::Escape { // TODO: 使用自定义类型解耦？
                dfa.feed(StateTrans::Resume);
                info!("Resumed.");
                break;
            }
        }
    }
}
