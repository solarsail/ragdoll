use specs::{RunArg, Gate, System, Join};

use game::render::{RenderBuffer_1, RenderCommand};
use components::{Renderable, Position};
use def::Point;


pub struct RenderSystem;

impl System<()> for RenderSystem {
    fn run(&mut self, arg: RunArg, _: ()) {
        let (mut object_buffer, renderables, positions) =
            arg.fetch(|w| {
                          (w.write_resource::<RenderBuffer_1>(),
                           w.read::<Renderable>(),
                           w.read::<Position>())
                      });
        for (r, p) in (&renderables, &positions).join() {
            object_buffer.push_back(RenderCommand {
                                        texture_id: r.tid.clone(),
                                        pos: Point::new(p.x, p.y),
                                        size: r.size.clone(),
                                        alpha: r.alpha,
                                    });
        }

    }
}
