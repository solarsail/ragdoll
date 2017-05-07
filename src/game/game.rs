use std::{thread, time};
use std::collections::HashMap;

use sdl2;
use sdl2::hint;
use sdl2::{Sdl, EventPump};
use sdl2::ttf::Sdl2TtfContext;
use sdl2::render::{Renderer, BlendMode};
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::mouse::MouseButton;
use sdl2::pixels::Color;
use sdl2::video::GLProfile;

use na::{Point2, Vector2};

use settings::*;
use map::*;
use rectgrid::{Vertex, Layout, Position, Vector};
use game::states::{OpeningState, GamePlayState, TitleState, PauseState};
use game::{StateTrans, StateMachine, GameState};
use resource::Resources;


const FPS: u32 = 60;

/// 每帧不同阶段间所使用或传递的状态与资源。
pub struct GameContext<'a> {
    pub cursor_screen_coord: Point2<i32>,
    pub key_states: HashMap<Keycode, bool>,
    pub key_triggers: Vec<Keycode>,
    pub mouse_states: HashMap<MouseButton, bool>,
    pub render_size: [u32; 2],
    pub scroll_rate: u32,
    pub res: Resources<'a>,
}

pub struct Game<'a, 'b: 'a> {
    ttf_ctx: &'a Sdl2TtfContext,
    renderer: &'a mut Renderer<'b>,
    event_pump: &'a mut EventPump,
    dfa: &'a mut StateMachine,
    states: &'a mut Vec<Box<GameState>>,
    running: bool,
    context: GameContext<'a>,
}

impl<'a, 'b> Game<'a, 'b> {
    pub fn start<F>(window_title: &str, settings: Settings, worker: F)
        where F: Fn(&mut Game) -> ()
    {

        info!("Initializing...");
        //hint::set("SDL_RENDER_SCALE_QUALITY", "2");
        let sdl_context = sdl2::init().unwrap();
        let video_subsystem = sdl_context.video().unwrap();
        let gl_attr = video_subsystem.gl_attr();
        gl_attr.set_context_profile(GLProfile::Core);
        gl_attr.set_context_version(3, 2);
        // Enable anti-aliasing
        gl_attr.set_multisample_buffers(1);
        gl_attr.set_multisample_samples(4);
        let window = video_subsystem
            .window(window_title, 800, 600)
            .position_centered()
            .resizable()
            .opengl()
            .build()
            .unwrap();
        let mut renderer = window.renderer().accelerated().build().unwrap(); // consumed window
        renderer.set_blend_mode(BlendMode::Blend);
        let mut event_pump = sdl_context.event_pump().unwrap();
        let ttf_context = sdl2::ttf::init().unwrap();

        // init state machine
        let mut states: Vec<Box<GameState>> = Vec::new();
        let mut dfa = StateMachine::new();

        let opening = dfa.add_state(&mut states,
                                    Box::new(OpeningState::new(4.0, 200, 200)),
                                    false);
        let title = dfa.add_state(&mut states, Box::new(TitleState::new()), false);
        let pause = dfa.add_state(&mut states, Box::new(PauseState::new()), false);
        let map = RectMap::test(5);

        let layout = Layout::new(Vector::new(200, 200), Vertex::new(0, 0), 20, 20);
        let gameplay = dfa.add_state(&mut states, Box::new(GamePlayState::new(map, layout)), true);

        dfa.set_initial(opening);
        dfa.add_trans(opening, title, StateTrans::Title);
        dfa.add_trans(title, gameplay, StateTrans::Gameplay);
        dfa.add_trans(gameplay, pause, StateTrans::Pause);
        dfa.add_trans(pause, gameplay, StateTrans::Resume);

        // create game
        let mut game = Game::new(settings,
                                 window_title,
                                 &ttf_context,
                                 &mut renderer,
                                 &mut event_pump,
                                 &mut dfa,
                                 &mut states);

        info!("Game started.");
        worker(&mut game);
    }

    pub fn new(settings: Settings,
               title: &str,
               ttf_ctx: &'a Sdl2TtfContext,
               renderer: &'a mut Renderer<'b>,
               event_pump: &'a mut EventPump,
               dfa: &'a mut StateMachine,
               states: &'a mut Vec<Box<GameState>>)
               -> Self {
        Game {
            context: GameContext {
                render_size: [settings.window_width, settings.window_height],
                cursor_screen_coord: Point2::new(0, 0),
                scroll_rate: settings.scroll_rate,
                res: Resources::new(ttf_ctx, renderer),
                key_states: HashMap::new(),
                key_triggers: Vec::new(),
                mouse_states: HashMap::new(),
            },
            ttf_ctx: ttf_ctx,
            renderer: renderer,
            event_pump: event_pump,
            dfa: dfa,
            states: states,
            running: true,
        }
    }

    fn process_events(&mut self) {
        let size = self.renderer.window().unwrap().size();
        self.context.render_size = [size.0, size.1];
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
                        Keycode::Up | Keycode::Down | Keycode::Left | Keycode::Right => {
                            self.context.key_states.insert(code, true);
                        }
                        _ => {
                            self.context.key_triggers.push(code);
                        }
                    }
                }
                Event::KeyUp { keycode: Some(code), .. } => {
                    match code {
                        Keycode::Up | Keycode::Down | Keycode::Left | Keycode::Right => {
                            self.context.key_states.insert(code, false);
                        }
                        _ => {}
                    }
                }
                Event::MouseMotion { x, y, .. } => {
                    self.context.cursor_screen_coord = Point2::new(x, y);
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
        self.renderer.clear();
        for i in self.dfa.ui_stack() {
            self.states[i].on_render(&mut self.context, &mut self.renderer);
        }
        self.renderer.present();
    }

    pub fn run(&mut self) {
        let time_per_frame: time::Duration = time::Duration::new(0, 1_000_000_000u32 / FPS);
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
                self.update(time_per_frame.as_secs() as f64 +
                            time_per_frame.subsec_nanos() as f64 / 1_000_000_000.0);
            }

            // 渲染
            self.render();

            let frame_time = time::Instant::now() - frame_start;
            if frame_time < time_per_frame {
                thread::sleep(time_per_frame - frame_time);
            }
        }
    }
}
