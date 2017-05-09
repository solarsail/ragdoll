use specs;
use game::Context;

pub struct RenderSystem;

impl specs::System<Context> for RenderSystem {
    fn run(&mut self, arg: specs::RunArg, ctx: Context) {}
}
