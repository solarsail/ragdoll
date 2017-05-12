use sdl2;
use sdl2::{Sdl, EventPump};
use sdl2::ttf::Sdl2TtfContext;
use sdl2::render::WindowCanvas;
use sdl2::event::Event;


pub struct Game<'a> {
    ttf_ctx: &'a Sdl2TtfContext,
    canvas: &'a mut WindowCanvas,
    event_pump: &'a mut EventPump,
    // TODO: assets: AssetManager,
    running: bool,
}

impl<'a> Game<'a> {
    pub fn start(window_title: &str, window_width: i32, window_height: i32) {
        let sdl_context = sdl2::init().unwrap();
        let video_subsystem = sdl_context.video().unwrap();

        let window = video_subsystem
            .window(window_title, window_width, window_height)
            .position_centered()
            .build()
            .unwrap();

        let mut canvas = window.into_canvas().accelerated().build().unwrap();
        canvas.set_draw_color(sdl2::pixels::Color::RGBA(0, 0, 0, 255));

        let ttf_ctx = sdl2::ttf::init().unwrap();

        let mut event_pump = sdl_context.event_pump().unwrap();

        let mut game = Game {
            ttf_ctx: &ttf_ctx,
            canvas: &mut canvas,
            event_pump: &mut event_pump,
            running: false,
        };

        game.run();
    }

    fn run(&mut self) {
        while self.running {
            for event in event_pump.poll_iter() {}
        }
    }
}
