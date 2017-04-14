use std::{thread, time};
use std::collections::HashMap;

use sdl2;
use sdl2::EventPump;
use sdl2::render::Renderer;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::mouse::MouseButton;
use sdl2::pixels::Color;

use settings::*;
use game::{GameState, StateMachine};
use game::states::*;
use resource::Resources;


const FPS: i32 = 60;

/// 每帧不同阶段间所使用或传递的状态与资源。
pub struct GameContext<'a> {
    pub cursor_screen_coord: [f64; 2],
    pub key_states: HashMap<Keycode, bool>,
    pub key_triggers: Vec<Keycode>,
    pub mouse_states: HashMap<MouseButton, bool>,
    pub render_size: [u32; 2],
    pub scroll_rate: u32,
    pub res: Resources,
}

pub struct Game<'a> {
    context: GameContext<'a>,
    dfa: &'a mut StateMachine,
    states: &'a mut Vec<Box<GameState>>,
    renderer: Renderer,
    event_pump: EventPump,
    running: bool,
}

impl<'a> Game<'a> {
    pub fn new(settings: Settings,
               title: &str,
               dfa: &'a mut StateMachine,
               states: &'a mut Vec<Box<GameState>>) -> Self {
        let sdl_context = sdl2::init().unwrap();
        let video_subsystem = sdl_context.video().unwrap();
        let window = video_subsystem
            .window(title, 800, 600)
            .position_centered()
            .opengl()
            .build()
            .unwrap();
        let renderer = window.renderer().build().unwrap();  // consumed window
        let event_pump = sdl_context.event_pump().unwrap();
        Game {
            context: GameContext {
                render_size: [settings.window_width, settings.window_height],
                cursor_screen_coord: [0.0, 0.0],
                scroll_rate: settings.scroll_rate,
                res: resource::Resources::new(&mut renderer),
            },
            renderer: renderer,
            event_pump: event_pump,
            dfa: dfa,
            states: states,
            running: true,
        }
    }

    fn process_events(&mut self) {
        self.context.render_size = [self.renderer.surface().unwrap().width, self.renderer.surface().unwrap().height];
        for event in self.event_pump.poll_iter() {
            match event {
                Event::Quit { .. } => {
                    self.running = false;
                }
                Event::AppDidEnterBackground { .. } => {
                    // 暂停
                    debug!("Entered background");
                }
                Event::KeyDown { keycode: Some(code), .. } => {
                    match code {
                        Keycode::Up | Keycode::Down | Keycode::Left | Keycode::Right => self.context.key_states.insert(code, true),
                        Keycode::Escape => self.context.key_triggers.push(code),
                        _ => {}
                    }
                }
                Event::KeyUp { keycode: Some(code), .. } => {
                    match code {
                        Keycode::Up | Keycode::Down | Keycode::Left | Keycode::Right => self.context.key_states.insert(code, false),
                        _ => {}
                    }
                }
                Event::MouseMotion { x, y, .. } => {
                    self.context.cursor_screen_coord = [x, y];
                }
                Event::MouseButtonDown { mouse_btn, .. } => {
                    self.context.mouse_states.insert(mouse_btn, true);
                }
                Event::MouseButtonUp { mouse_btn, .. } => {
                    self.context.mouse_states.insert(mouse_btn, false);
                }
                _ => {}
            }
        }
        let current = self.dfa.current_state_id();
        self.states[current].on_input(&mut self.context, &mut self.dfa);
        self.context.key_triggers.clear();
    }

    fn update(&mut self, dt: f64) {
        let current = self.dfa.current_state_id();
        self.states[current].on_update(&mut self.context, &mut self.dfa, dt);
    }

    fn render(&mut self) {
        self.renderer.set_draw_color(Color::RGB(0, 0, 0));
        self.render.clear();
        for i in self.dfa.ui_stack() {
            self.states[i].on_render(&mut self.context, &mut self.renderer);
        }
        self.renderer.present();
    }

    pub fn run(&mut self) {
        let time_per_frame: time::Duration = time::Duration::new(0, 1_000_000_000 / FPS);
        let mut time_since_last_update = time::Duration::from_millis(0);
        let mut mark = time::Instant::now();

        self.renderer.set_draw_color(Color::RGB(0, 0, 0));
        self.renderer.clear();
        self.renderer.present();

        while self.running {
            let frame_start = time::Instant::now();

            // 处理事件
            self.process_events();

            // 更新状态
            time_since_last_update += time::Instant::now() - mark;
            mark = time::Instant::now();

            while time_since_last_update > time_per_frame {
                time_since_last_update -= time_per_frame;
                self.process_events();
                self.update(time_per_frame.as_secs() as f64 + time_per_frame.subsec_nanos() as f64 / 1_000_000_000.0);
            }
            
            // 渲染
            self.render();

            let frame_time = time::Instant::now() - frame_start;
            if frame_time < time_per_frame {
                std::thread::sleep(time_per_frame - frame_time);
            }
        }
    }

}
