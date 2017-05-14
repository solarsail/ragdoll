use specs;

pub struct RenderSystem;

impl specs::System<()> for RenderSystem {
    fn run(&mut self, arg: specs::RunArg, _: ()) {}
}
