use std::sync::{Mutex, Arc};

use amethyst::context::Context;
use amethyst::ecs::{World, Join, VecStorage, Component, Processor, RunArg};


struct MapProcessor {
    field: Type
}

impl Processor<Arc<Mutex<Context>>> for MapProcessor {
}
