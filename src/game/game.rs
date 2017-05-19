use std::sync::Arc;
use std::{thread, time};

use threadpool::ThreadPool;
use num_cpus;

use sdl2;
use sdl2::rect::Rect;
use sdl2::EventPump;
use sdl2::ttf::Sdl2TtfContext;
use sdl2::render::WindowCanvas;
use sdl2::event::Event;
use specs::{Planner, World, Gate};

use game::states;
use game::{InputHandler, StateMachine};
use game::render::{RenderBuffer_0, RenderBuffer_1};
use resource::AssetManager;
use components::{Renderable, Position};
use systems::RenderSystem;


const FPS: u32 = 60;

pub struct Game<'a, 'b> {
    canvas: &'a mut WindowCanvas,
    event_pump: &'a mut EventPump,
    assets: AssetManager<'b>,
    state_machine: StateMachine,
    planner: Planner<()>,
    last_update: time::Instant,
    accumulated_delta: time::Duration,
    time_per_frame: time::Duration,
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
        let tile_buffer = RenderBuffer_0::new();
        let object_buffer = RenderBuffer_1::new();
        let mut world = World::new();
        world.add_resource::<InputHandler>(input_handler);
        world.add_resource::<RenderBuffer_0>(tile_buffer);
        world.add_resource::<RenderBuffer_1>(object_buffer);
        world.register::<Renderable>();
        world.register::<Position>();
        // planner
        let pool = Arc::new(ThreadPool::new(num_cpus::get()));
        let mut planner = Planner::from_pool(world, pool);
        // systems
        let render_sys = RenderSystem;
        planner.add_system(render_sys, "render", 0);
        // state machine
        let opening = states::OpeningState::new(4.0);
        let state_machine = StateMachine::new(opening);

        let mut game = Game {
            canvas: &mut canvas,
            event_pump: &mut event_pump,
            assets,
            state_machine,
            planner: planner,
            last_update: time::Instant::now(),
            accumulated_delta: time::Duration::new(0, 0),
            time_per_frame: time::Duration::new(0, 1_000_000_000u32 / FPS),
            running: true,
        };

        game.run();
    }

    fn run(&mut self) {
        self.state_machine
            .start(self.planner.mut_world(), &mut self.assets);

        while self.running {
            let frame_start = time::Instant::now();
            self.accumulated_delta += time::Instant::now() - self.last_update;
            self.last_update = time::Instant::now();
            // handle event
            self.handle_event();
            // update
            self.update();
            // render
            self.render();


            let frame_time = time::Instant::now() - frame_start;
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
    }

    fn render(&mut self) {
        self.canvas.clear();
        {
            let mut tile_buffer = self.planner
                .mut_world()
                .write_resource::<RenderBuffer_0>()
                .pass();
            while let Some(c) = tile_buffer.pop_front() {
                let rect = Rect::new(c.pos.x, c.pos.y, c.size.w, c.size.h);
                let texture = self.assets.texture(&c.texture_id);
                texture.borrow_mut().set_alpha_mod(c.alpha);
                self.canvas.copy(&texture.borrow(), None, rect).unwrap();
            }
        }
        {
            let mut object_buffer = self.planner
                .mut_world()
                .write_resource::<RenderBuffer_1>()
                .pass();
            while let Some(c) = object_buffer.pop_front() {
                let rect = Rect::new(c.pos.x, c.pos.y, c.size.w, c.size.h);
                let texture = self.assets.texture(&c.texture_id);
                //&mut texture.set_alpha_mod(c.alpha);
                texture.borrow_mut().set_alpha_mod(c.alpha);
                self.canvas.copy(&texture.borrow(), None, rect).unwrap();
            }
        }
        self.canvas.present();
    }
}
