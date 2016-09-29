extern crate amethyst;

use amethyst::engine::Application;
use amethyst::processors::rendering::{RenderingProcessor, Renderable, Light, Camera};
use amethyst::context::Context;
use amethyst::config::Element;

mod geometry;
mod mesh;
mod game;
mod map;
mod settings;
mod processors;

use processors::InputProcessor;
use map::Coordinates;



fn main() {
    use amethyst::engine::Config;
    let path = format!("{}/resources/config.yml",
                       env!("CARGO_MANIFEST_DIR"));
    let config = Config::from_file(path).unwrap();
    let mut context = Context::new(config.context_config);
    let rendering_processor = RenderingProcessor::new(config.renderer_config, &mut context);
    let mut game = Application::build(game::Game, context)
                   .with::<RenderingProcessor>(rendering_processor, "rendering_processor", 0)
                   .register::<Renderable>()
                   .register::<Light>()
                   .register::<Camera>()
                   .with::<InputProcessor>(InputProcessor, "game_processor", 1)
                   .register::<Coordinates>()
                   .done();
    game.run();
}