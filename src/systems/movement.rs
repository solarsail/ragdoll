use specs::{RunArg, System, Join, Gate};

use game::render::{RenderBuffer1, RenderCommand};
use game::input::{InputHandler, Keycode};
use game::GameClock;
use components::{Renderable, Position, Text, MainCamera, InputReceiver};
use cgmath::{Vector2, InnerSpace};


pub struct MovementSystem;

impl System<()> for MovementSystem {
    fn run(&mut self, arg: RunArg, _: ()) {
        let (mut input_handler, clock, mut ps, rs) =
            arg.fetch(|w| {
                          (w.write_resource::<InputHandler>(),
                           w.read_resource::<GameClock>(),
                           w.write::<Position>(),
                           w.read::<InputReceiver>())
                      });
        let mut v: Vector2<f32> = Vector2::new(0.0, 0.0);
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
            v = v.normalize_to(100.0 * clock.dt); // use meters instead of pixels?
        }
        //println!("{:?}", v);

        for (mut p, _) in (&mut ps, &rs).join() {
            p.x += v.x;
            p.y += v.y;
        }
    }
}
