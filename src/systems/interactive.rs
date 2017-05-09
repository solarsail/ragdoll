use specs;
use game::Context;

pub struct InteractiveSystem;

impl specs::System<Context> for InteractiveSystem {
    fn run(&mut self, arg: specs::RunArg, ctx: Context) {}
}
