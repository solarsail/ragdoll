use std::sync::Arc;
use std::{thread, time};

use threadpool::ThreadPool;
use num_cpus;

use sdl2;
use sdl2::EventPump;
use sdl2::ttf::Sdl2TtfContext;
use sdl2::render::WindowCanvas;
use sdl2::event::Event;
use specs::{Planner, World, Gate};

use game::states;
use game::{InputHandler, StateMachine};
use resource::AssetManager;
use components::{Renderable, Position};


const FPS: u32 = 60;

pub struct Game<'a> {
    ttf_ctx: &'a Sdl2TtfContext,
    canvas: &'a mut WindowCanvas,
    event_pump: &'a mut EventPump,
    assets: AssetManager<'a, 'a>,
    state_machine: StateMachine,
    planner: Planner<()>,
    running: bool,
}

impl<'a> Game<'a> {
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
        let assets = AssetManager::new(&ttf_ctx, &canvas);
        // event pump
        let mut event_pump = sdl_context.event_pump().unwrap();
        // resources
        let input_handler = InputHandler::new();
        let mut world = World::new();
        world.add_resource::<InputHandler>(input_handler);
        world.register::<Renderable>();
        world.register::<Position>();
        // planner
        let pool = Arc::new(ThreadPool::new(num_cpus::get()));
        let mut planner = Planner::from_pool(world, pool);
        // state machine
        let opening = states::OpeningState::new();
        let state_machine = StateMachine::new(opening);

        let mut game = Game {
            ttf_ctx: &ttf_ctx,
            canvas: &mut canvas,
            event_pump: &mut event_pump,
            assets,
            state_machine,
            planner: planner,
            running: true,
        };

        game.run();
    }

    fn run(&mut self) {
        let time_per_frame = time::Duration::new(0, 1_000_000_000u32 / FPS);
        let mut time_since_last_update = time::Duration::from_millis(0);
        let mut mark = time::Instant::now();
        self.state_machine
            .start(self.planner.mut_world(), &mut self.assets);

        //let input_handler = input_handler_lock.deref_mut();
        while self.running {
            let frame_start = time::Instant::now();
            time_since_last_update += time::Instant::now() - mark;
            mark = time::Instant::now();
            {
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
                                    .handle_event(&event,
                                                  self.planner.mut_world(),
                                                  &mut self.assets)
                            }
                        }
                    }
                }
            }
            // update
            loop {
                self.state_machine
                    .update(self.planner.mut_world(),
                            &mut self.assets,
                            time_per_frame.as_secs() as f32 +
                            time_per_frame.subsec_nanos() as f32 / 1_000_000_000.0);
                if time_since_last_update > time_per_frame {
                    time_since_last_update -= time_per_frame;
                } else {
                    break;
                }
            }
            self.planner.dispatch(());
            // TODO: render

            let frame_time = time::Instant::now() - frame_start;
            if frame_time < time_per_frame {
                thread::sleep(time_per_frame - frame_time);
            }
        }
    }
}
