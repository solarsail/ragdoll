use std::{thread, time};
use std::collections::HashMap;

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


const FPS: i32 = 60;

/// 每帧不同阶段间所使用或传递的状态与资源。
pub struct GameContext<'a> {
    pub cursor_screen_coord: [f64; 2],
    pub key_states: HashMap<Keycode, bool>,
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

    fn process_events(&mut self) {
        for event in self.event_pump.poll_iter() {
            match event {
                Event::Quit { .. } => {
                    break;
                }
                Event::AppDidEnterBackground { .. } => {
                    // 暂停
                }
                Event::KeyDown { keycode, .. } => {

                }
                _ => {
                    let current = self.dfa.current_state_id();
                    self.states[current].on_input(&mut self.context, &mut self.dfa);
                }
            }
        }
    }

    fn update(&mut self, dt: f64) {
        let current = self.dfa.current_state_id();
        self.states[current].on_update(&mut self.context, &mut self.dfa, dt);
    }

    fn render(&mut self) {
        for i in self.dfa.ui_stack() {
            self.states[i].on_render(&mut self.context, &mut self.renderer);
        }
    }

    pub fn run(&mut self) {
        let time_per_frame: time::Duration = time::Duration::new(0, 1_000_000_000 / FPS);
        let mut time_since_last_update = time::Duration::from_millis(0);
        let mut mark = time::Instant::now();

        renderer.set_draw_color(Color::RGB(255, 255, 255));
        renderer.clear();
        renderer.present();

        loop {
            let frame_start = time::Instant::now();

            // 处理事件
            self.process_events();

            // 更新状态
            time_since_last_update += (time::Instant::now() - mark);
            mark = time::Instant::now();

            while time_since_last_update > time_per_frame {
                time_since_last_update -= time_per_frame;
                self.process_events();
                self.update(time_per_frame.as_secs() as f64 + time_per_frame.subsec_nanos() as f64 / 1_000_000_000.0);
            }
            
            // 渲染
            self.render();

            let frame_time = time::Instant::now() - frame_start;
            if frame_time < millis_per_frame {
                std::thread::sleep(millis_per_frame - frame_time);
            }
        }
    }

}
