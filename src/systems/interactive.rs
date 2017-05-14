use specs;

pub struct InteractiveSystem;

impl specs::System<()> for InteractiveSystem {
    fn run(&mut self, arg: specs::RunArg, _: ()) {}
}
