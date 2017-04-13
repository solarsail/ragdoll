use sdl2;
use sdl2::EventPump;
use sdl2::video::Window;
use sdl2::render::Renderer;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;

use settings::*;
use game::{GameState, StateMachine};
use game::states::*;
use resource::Resources;

/// 每帧不同阶段间所使用或传递的状态与资源。
pub struct GameContext<'a> {
    pub cursor_screen_coord: [f64; 2],
    pub render_size: [u32; 2],
    pub scroll_rate: u32,
    pub res: &'a mut Resources,
}

pub struct Game<'a> {
    context: GameContext<'a>,
    dfa: &'a mut StateMachine,
    window: Window,
    states: &'a mut Vec<Box<GameState>>,
    renderer: Renderer,
    event_pump: EventPump,
}

impl<'a> Game<'a> {
    pub fn new(settings: Settings,
               title: &str,
               res: &'a mut Resources,
               dfa: &'a mut StateMachine,
               states: &'a mut Vec<Box<GameState>>) -> Self {
        let sdl_context = sdl2::init().unwrap();
        let video_subsystem = sdl_context.video().unwrap();
        let window = video_subsystem
            .window(title, 800, 600)
            .position_centered()
            .opengl()
            .build()
            .unwrap(),
        let renderer = window.renderer().build().unwrap();
        let event_pump = sdl_context.event_pump().unwrap();
        Game {
            context: GameContext {
                render_size: [settings.window_width, settings.window_height],
                cursor_screen_coord: [0.0, 0.0],
                scroll_rate: settings.scroll_rate,
                res: res,
            },
            window: window,
            renderer: renderer,
            event_pump: event_pump,
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
        loop {
            // 处理事件
            for event in self.event_pump.poll_iter() {
                match event {
                    Event::Quit { .. } => {
                        break;
                    }
                    Event::AppDidEnterBackground { .. } => {
                        // 暂停
                    }
                    _ => {
                        let current = self.dfa.current_state_id();
                        self.states[current].on_input(&mut self.context, &mut self.dfa, &e);
                    }
                }
            }
            // 更新状态
            let current = self.dfa.current_state_id();
            self.states[current].on_update(&mut self.context, &mut self.dfa, dt);
            // 渲染
            for i in self.dfa.ui_stack() {
                self.states[i].on_render(&mut self.context, &e, self.window);
            }
        }
    }

}
