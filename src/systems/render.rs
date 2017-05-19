use specs::{RunArg, System, Join};

use game::render::{RenderBuffer1, RenderCommand};
use components::{Renderable, Position, Text};
use def::Point;


pub struct RenderSystem;

impl System<()> for RenderSystem {
    fn run(&mut self, arg: RunArg, _: ()) {
        let (mut object_buffer, renderables, texts, positions) =
            arg.fetch(|w| {
                          (w.write_resource::<RenderBuffer1>(),
                           w.read::<Renderable>(),
                           w.read::<Text>(),
                           w.read::<Position>())
                      });
        for (r, p) in (&renderables, &positions).join() {
            object_buffer.push_back(RenderCommand::Texture {
                                        texture_id: r.tid.clone(),
                                        pos: Point::new(p.x, p.y),
                                        size: Some(r.size),
                                        alpha: Some(r.alpha),
                                    });
        }
        for (t, p) in (&texts, &positions).join() {
            object_buffer.push_back(RenderCommand::Text {
                                        font_id: t.fid.clone(),
                                        content: t.content.clone(),
                                        width: t.max_width,
                                        pos: Point::new(p.x, p.y),
                                        color: t.color,
                                    });
        }

    }
}
