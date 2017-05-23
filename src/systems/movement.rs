use specs::{RunArg, System, Join};

use game::render::{RenderBuffer1, RenderCommand};
use components::{Renderable, Position, Text, MainCamera, InputReceiver};
use def::Point;


pub struct MovementSystem;

impl System<()> for MovementSystem {
    fn run(&mut self, arg: RunArg, _: ()) {
        let (mut input_handler, dt, ps, rs) = arg.fetch(|w| {
                                                            (w.write_resource::<InputHandler>(),
                                                             w.read_resource::<DeltaTime>(),
                                                             w.write::<Position>(),
                                                             w.read::<InputReceiver>())
                                                        });
        let dt = dt.pass();
        let input_handler = input_handler.pass();
        let mut v = Vector2::new(0.0, 0.0);
        if input_handler.key_down(Keycode::W) {
            v.y -= 1.0;
        } else if input_handler.key_down(Keycode::S) {
            v.y += 1.0;
        }
        if input_handler.key_down(Keycode::A) {
            v.x -= 1.0;
        } else if input_handler.key_down(Keycode::D) {
            v.x += 1.0;
        }
        if v.magnitude2() > 0.0 {
            v = v.normalize_to(100.0 * dt); // use meters instead of pixels?
        }

        for (mut p, r) in (&mut ps.pass(), &rs.pass()).join() {
            p.x += v.x as i32;
            p.y += v.y as i32;
        }
    }
}
