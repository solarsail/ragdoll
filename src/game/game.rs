use std::sync::Arc;
use std::{thread, time};
use std::env;

use threadpool::ThreadPool;
use num_cpus;

use sdl2;
use sdl2::rect::Rect;
use sdl2::EventPump;
use sdl2::render::WindowCanvas;
use sdl2::event::Event;
use sdl2::pixels::Color;
use specs::{Planner, World, Gate};

use game::states;
use game::{InputHandler, StateMachine};
use game::render::{RenderBuffer0, RenderBuffer1, RenderCommand, ScreenDimension};
use resource::AssetManager;
use components::{Renderable, Position, Text};
use systems::RenderSystem;


const FPS: u32 = 60;

pub struct Game<'a, 'b> {
    canvas: &'a mut WindowCanvas,
    event_pump: &'a mut EventPump,
    assets: AssetManager<'b>,
    state_machine: StateMachine,
    planner: Planner<()>,
    start_instant: time::Instant,
    last_update: time::Instant,
    accumulated_delta: time::Duration,
    time_per_frame: time::Duration,
    frame_counter: u32, // 在每秒100帧的情况下，连续运行约500天将溢出
    running: bool,
}

impl<'a, 'b> Game<'a, 'b> {
    pub fn start(window_title: &str, window_width: u32, window_height: u32) {
        let sdl_context = sdl2::init().unwrap();
        let video_subsystem = sdl_context.video().unwrap();
        let window = video_subsystem
            .window(window_title, window_width, window_height)
            .position_centered()
            .build()
            .unwrap();
        // canvas
        let mut canvas = window.into_canvas().accelerated().build().unwrap();
        canvas.set_draw_color(sdl2::pixels::Color::RGBA(0, 0, 0, 255));
        // assets
        let ttf_ctx = sdl2::ttf::init().unwrap();
        let texture_creator = canvas.texture_creator();
        let assets = AssetManager::new(&texture_creator, &ttf_ctx);
        // event pump
        let mut event_pump = sdl_context.event_pump().unwrap();
        // resources
        let input_handler = InputHandler::new();
        let tile_buffer = RenderBuffer0::new();
        let object_buffer = RenderBuffer1::new();
        let screen_dim = ScreenDimension::new(window_width, window_height);
        let mut world = World::new();
        world.add_resource::<InputHandler>(input_handler);
        world.add_resource::<RenderBuffer0>(tile_buffer);
        world.add_resource::<RenderBuffer1>(object_buffer);
        world.add_resource::<ScreenDimension>(screen_dim);
        world.register::<Renderable>();
        world.register::<Text>();
        world.register::<Position>();
        // planner
        let pool = Arc::new(ThreadPool::new(num_cpus::get()));
        let mut planner = Planner::from_pool(world, pool);
        // systems
        let render_sys = RenderSystem;
        planner.add_system(render_sys, "render", 0);
        // state machine
        let opening = states::OpeningState::new(8.0);
        let state_machine = StateMachine::new(opening);

        let mut game = Game {
            canvas: &mut canvas,
            event_pump: &mut event_pump,
            assets,
            state_machine,
            planner: planner,
            start_instant: time::Instant::now(),
            last_update: time::Instant::now(),
            accumulated_delta: time::Duration::new(0, 0),
            time_per_frame: time::Duration::new(0, 1_000_000_000u32 / FPS),
            frame_counter: 0,
            running: true,
        };

        game.run();
    }

    fn run(&mut self) {
        self.state_machine
            .start(self.planner.mut_world(), &mut self.assets);

        while self.running {
            self.frame_counter += 1;

            let frame_start = time::Instant::now();
            self.accumulated_delta += self.last_update.elapsed();
            self.last_update = time::Instant::now();
            // handle event
            self.handle_event();
            // update
            self.update();
            // render
            self.render();

            let frame_time = frame_start.elapsed();
            if frame_time < self.time_per_frame {
                thread::sleep(self.time_per_frame - frame_time);
            }
        }
    }

    fn handle_event(&mut self) {
        let mut input_handler = self.planner
            .mut_world()
            .write_resource::<InputHandler>()
            .pass();
        // Event handling
        for event in self.event_pump.poll_iter() {
            let processed = input_handler.update(&event);
            if !processed {
                match event {
                    Event::Quit { .. } => {
                        self.running = false;
                    }
                    _ => {
                        self.state_machine
                            .handle_event(&event, self.planner.mut_world(), &mut self.assets)
                    }
                }
            }
        }
    }

    fn update(&mut self) {
        loop {
            self.state_machine
                .update(self.planner.mut_world(),
                        &mut self.assets,
                        self.time_per_frame.as_secs() as f32 +
                        self.time_per_frame.subsec_nanos() as f32 / 1_000_000_000.0);
            if self.accumulated_delta > self.time_per_frame {
                self.accumulated_delta -= self.time_per_frame;
            } else {
                break;
            }
        }
        self.planner.dispatch(());
        // TODO: update screen dimisions
    }

    fn _render(&mut self, cmd: RenderCommand) {
        match cmd {
            RenderCommand::Texture {
                texture_id,
                pos,
                size,
                alpha,
            } => {
                let texture = self.assets.texture(&texture_id);
                let rect = if let Some(s) = size {
                    Rect::new(pos.x, pos.y, s.w, s.h)
                } else {
                    let q = texture.borrow().query();
                    Rect::new(pos.x, pos.y, q.width, q.height)
                };
                if let Some(a) = alpha {
                    texture.borrow_mut().set_alpha_mod(a);
                }
                self.canvas.copy(&texture.borrow(), None, rect).unwrap();
            }
            RenderCommand::Text {
                font_id,
                content,
                pos,
                width,
                color,
            } => {
                let texture = self.assets.text(&font_id, &content, width, color);
                let q = texture.borrow().query();
                let rect = Rect::new(pos.x, pos.y, q.width, q.height);
                self.canvas.copy(&texture.borrow(), None, rect).unwrap();
            }
        }
    }
    fn render(&mut self) {
        self.canvas.clear();
        {
            let mut tile_buffer = self.planner
                .mut_world()
                .write_resource::<RenderBuffer0>()
                .pass();
            while let Some(c) = tile_buffer.pop_front() {
                self._render(c);
            }
        }
        {
            let mut object_buffer = self.planner
                .mut_world()
                .write_resource::<RenderBuffer1>()
                .pass();
            while let Some(c) = object_buffer.pop_front() {
                self._render(c);
            }
        }
        // FPS counter. TODO: add a switch
        if env::var("SHOW_FPS").is_ok() {
            self._draw_fps();
        }

        self.canvas.present();
    }

    fn _draw_fps(&mut self) {
        let elapsed = self.start_instant.elapsed();
        let sec = elapsed.as_secs() as f32 + elapsed.subsec_nanos() as f32 / 1_000_000_000.0;
        let fps = self.assets
            .text_uncached("debug",
                           &format!("FPS: {:.2}", self.frame_counter as f32 / sec),
                           Color::RGB(255, 0, 0));
        let q = fps.borrow().query();
        let rect = Rect::new(10, 10, q.width, q.height);
        self.canvas.copy(&fps.borrow(), None, rect).unwrap();
    }
}
