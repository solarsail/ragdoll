use sdl2::render::Renderer;
use sdl2::keyboard::Keycode;
use sdl2::mouse::MouseButton;
use map::*;
use game::{GameContext, GameState, StateTrans, StateMachine};
use view::View;
use rectgrid::Layout;

#[derive(PartialEq, Eq)]
enum Scroll {
    None,
    Left,
    Right,
    Up,
    Down,
}

pub struct GamePlayState {
    map: RectMap,
    layout: Layout,
    cursor_region: Region,
    scroll: [Scroll; 2],
    mouse_scroll_lock: bool,
    map_view: View,
    ui_view: View,
}

impl GamePlayState {
    pub fn new(map: RectMap, layout: Layout) -> GamePlayState {
        GamePlayState {
            map: map,
            layout: layout,
            cursor_region: Region::new(Category::Neutral),
            scroll: [Scroll::None, Scroll::None],
            mouse_scroll_lock: false,
            map_view: View::new(),
            ui_view: View::new(),
        }
    }
}

impl GameState for GamePlayState {
    fn on_update(&mut self,
                 ctx: &mut GameContext,
                 dfa: &mut StateMachine,
                 dt: f64 /* in seconds */) {
        self.map_view
            .set_size(ctx.render_size[0], ctx.render_size[1]);
        let ds = ctx.scroll_rate as f64 * dt;
        match self.scroll[0] {
            Scroll::Left => {
                //self.map_view.trans_self(-ds, 0.0);
            }
            Scroll::Right => {
                //self.map_view.trans_self(ds, 0.0);
            }
            _ => {}
        }
        match self.scroll[1] {
            Scroll::Up => {
                //self.map_view.trans_self(0.0, -ds);
            }
            Scroll::Down => {
                //self.map_view.trans_self(0.0, ds);
            }
            _ => {}
        }
    }

    #[allow(unused_variables)]
    fn on_render(&mut self, ctx: &mut GameContext, r: &mut Renderer) {
        // TODO: culling: use view or draw_state.scissor? how to use it?
        self.map.draw(ctx, &self.layout, &self.map_view, r);
        //self.cursor_region.draw(&self.layout, &self.map_view, r);
    }

    fn on_input(&mut self, ctx: &mut GameContext, dfa: &mut StateMachine) {
        for key in ctx.key_triggers.iter() {
            if *key == Keycode::Escape {
                dfa.feed(StateTrans::Pause);
                info!("Paused.");
            }
        }

        if let Some(&state) = ctx.key_states.get(&Keycode::Left) {
            self.scroll[0] = if state { Scroll::Left } else { Scroll::None };
        }
        if let Some(&state) = ctx.key_states.get(&Keycode::Right) {
            self.scroll[0] = if state { Scroll::Right } else { Scroll::None };
        }
        if let Some(&state) = ctx.key_states.get(&Keycode::Up) {
            self.scroll[1] = if state { Scroll::Up } else { Scroll::None };
        }
        if let Some(&state) = ctx.key_states.get(&Keycode::Down) {
            self.scroll[1] = if state { Scroll::Down } else { Scroll::None };
        }
        if let Some(&state) = ctx.mouse_states.get(&MouseButton::Left) {
            if state {
                // TODO: use nalgebra to make translation
                //let cursor_world_coord = transform_pos(
                //    self.map_view.s2w_trans, ctx.cursor_screen_coord);
                //let hex = self.layout.coord_at(ctx.cursor_world_coord);
                //self.cursor_region.push(hex);
            } else {
                self.cursor_region.clear();
            }
        }
    }
}
