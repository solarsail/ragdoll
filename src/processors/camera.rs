use std::sync::{Mutex, Arc};

use amethyst::context::Context;
use amethyst::ecs::{Join, Processor, RunArg};
use amethyst::processors::rendering::Camera;

use processors::input::{InputState, InputEvents, InputEvent};
use settings::Settings;


pub struct CameraProcessor;

unsafe impl Sync for CameraProcessor {  }

impl Processor<Arc<Mutex<Context>>> for CameraProcessor {
    fn run(&mut self, arg: RunArg, _: Arc<Mutex<Context>>) {
        let (mut cameras,
             input_state,
             mut input_events,
             settings) = arg.fetch(|w| (w.write::<Camera>(),
                                        w.read_resource::<InputState>(),
                                        w.write_resource::<InputEvents>(),
                                        w.read_resource::<Settings>()));

        let cameras = &mut cameras;
        for camera in cameras.iter() {
            if input_state.up && !input_state.down {
                camera.eye[1] += settings.scroll_rate;
                camera.target[1] += settings.scroll_rate;
            } else if input_state.down && !input_state.up {
                camera.eye[1] -= settings.scroll_rate;
                camera.target[1] -= settings.scroll_rate;
            }
            if input_state.left && !input_state.right {
                camera.eye[0] -= settings.scroll_rate;
                camera.target[0] -= settings.scroll_rate;
            } else if input_state.right && !input_state.left {
                camera.eye[0] += settings.scroll_rate;
                camera.target[0] += settings.scroll_rate;
            }
            if input_state.zoomin && !input_state.zoomout {
                camera.eye[2] -= settings.scroll_rate;
                //camera.target[2] -= settings.scroll_rate;
            } else if input_state.zoomout && !input_state.zoomin {
                camera.eye[2] += settings.scroll_rate;
                //camera.target[2] += settings.scroll_rate;
            }

            while let Some(event) = input_events.camera.pop_front() {
                match event {
                    InputEvent::PrintCameraInfo  => {
                        println!("{:?}, {:?}", camera.eye, camera.target);
                    }
                    _ => {
                        println!("{:?}", event);
                    }
                }
            }
        }
    }
}