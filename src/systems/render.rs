use specs::{RunArg, System, Join};

use game::render::{RenderBuffer1, RenderCommand};
use components::{Renderable, Position, Text, MainCamera};
use def::Point;


pub struct RenderSystem;

impl System<()> for RenderSystem {
    fn run(&mut self, arg: RunArg, _: ()) {
        let (mut object_buffer, renderables, texts, positions, cameras) =
            arg.fetch(|w| {
                          (w.write_resource::<RenderBuffer1>(),
                           w.read::<Renderable>(),
                           w.read::<Text>(),
                           w.read::<Position>(),
                           w.read::<MainCamera>())
                      });
        let mut camera_origin = Point::new(0, 0);
        for (c, p) in (&cameras, &positions).join() {
            // TODO: in case at map edges
            camera_origin = Point::new(p.x - c.size.w as i32 / 2, p.y - c.size.h as i32 / 2);
        }
        for (r, p) in (&renderables, &positions).join() {
            object_buffer.push_back(RenderCommand::Texture {
                                        texture_id: r.tid.clone(),
                                        pos: Point::new(p.x - camera_origin.x,
                                                        p.y - camera_origin.y),
                                        size: Some(r.size),
                                        alpha: Some(r.alpha),
                                    });
        }
        for (t, p) in (&texts, &positions).join() {
            object_buffer.push_back(RenderCommand::Text {
                                        font_id: t.fid.clone(),
                                        content: t.content.clone(),
                                        width: t.max_width,
                                        pos: Point::new(p.x - camera_origin.x,
                                                        p.y - camera_origin.y),
                                        color: t.color,
                                    });
        }

    }
}
