use std::sync::{Mutex, Arc};

use amethyst::context::Context;
use amethyst::ecs::{Join, Processor, RunArg};
use amethyst::processors::rendering::{Camera};

use settings::Settings;

pub struct InputState {
    pub up: bool,
    pub down: bool,
    pub left: bool,
    pub right: bool,
    pub zoomin: bool,
    pub zoomout: bool,
    pub lmb: bool,
    pub rmb: bool,
    pub cursor_pos: (i32, i32),
    pub lmb_pos: (i32, i32),
    pub rmb_pos: (i32, i32),
}

impl InputState {
    pub fn new() -> InputState {
        InputState {
            up: false,
            down: false,
            left: false,
            right: false,
            zoomin: false,
            zoomout: false,
            lmb: false,
            rmb: false,
            cursor_pos: (-1, -1),
            lmb_pos: (-1, -1),
            rmb_pos: (-1, -1),
        }
    }
}


pub struct InputProcessor;

unsafe impl Sync for InputProcessor {  }

impl Processor<Arc<Mutex<Context>>> for InputProcessor {
    fn run(&mut self, arg: RunArg, context: Arc<Mutex<Context>>) {
        use amethyst::context::event::{EngineEvent, Event, VirtualKeyCode, MouseButton, ElementState};

        // Get all needed component storages and resources
        let context = context.lock().unwrap();
        let (mut cameras,
             mut input_state,
             settings) = arg.fetch(|w| (w.write::<Camera>(),
                                         w.write_resource::<InputState>(),
                                         w.read_resource::<Settings>()));

        let mut left_clicked: Option<(i32, i32)> = None;
        let mut right_clicked: Option<(i32, i32)> = None;
        let mut print_info = false;
        // Update input_state resource using incoming events
        let engine_events = context.broadcaster.read::<EngineEvent>();
        for engine_event in engine_events.iter() {
            match engine_event.payload {
                Event::KeyboardInput(ElementState::Pressed, _, Some(VirtualKeyCode::Up)) => input_state.up = true,
                Event::KeyboardInput(ElementState::Pressed, _, Some(VirtualKeyCode::Down)) => input_state.down = true,
                Event::KeyboardInput(ElementState::Released, _, Some(VirtualKeyCode::Up)) => input_state.up = false,
                Event::KeyboardInput(ElementState::Released, _, Some(VirtualKeyCode::Down)) => input_state.down = false,

                Event::KeyboardInput(ElementState::Pressed, _, Some(VirtualKeyCode::Left)) => input_state.left = true,
                Event::KeyboardInput(ElementState::Pressed, _, Some(VirtualKeyCode::Right)) => input_state.right = true,
                Event::KeyboardInput(ElementState::Released, _, Some(VirtualKeyCode::Left)) => input_state.left = false,
                Event::KeyboardInput(ElementState::Released, _, Some(VirtualKeyCode::Right)) => input_state.right = false,

                Event::KeyboardInput(ElementState::Pressed, _, Some(VirtualKeyCode::A)) => input_state.zoomin = true,
                Event::KeyboardInput(ElementState::Pressed, _, Some(VirtualKeyCode::Z)) => input_state.zoomout = true,
                Event::KeyboardInput(ElementState::Released, _, Some(VirtualKeyCode::A)) => input_state.zoomin = false,
                Event::KeyboardInput(ElementState::Released, _, Some(VirtualKeyCode::Z)) => input_state.zoomout = false,

                Event::KeyboardInput(ElementState::Released, _, Some(VirtualKeyCode::P)) => print_info = true,

                Event::MouseMoved(x, y) => input_state.cursor_pos = (x, y),
                Event::MouseInput(ElementState::Pressed, MouseButton::Left) => {
                    input_state.lmb = true;
                    input_state.lmb_pos = input_state.cursor_pos;
                }
                Event::MouseInput(ElementState::Pressed, MouseButton::Right) => {
                    input_state.rmb = true;
                    input_state.rmb_pos = input_state.cursor_pos;
                }
                Event::MouseInput(ElementState::Released, MouseButton::Left) => {
                    input_state.lmb = false;
                    if input_state.cursor_pos == input_state.lmb_pos {
                        left_clicked = Some(input_state.cursor_pos);
                    }
                }
                Event::MouseInput(ElementState::Released, MouseButton::Right) => {
                    input_state.rmb = false;
                    if input_state.cursor_pos == input_state.rmb_pos {
                        right_clicked = Some(input_state.cursor_pos);
                    }
                }
                
                _ => (),
            }
        }


        //let delta_time = context.delta_time.subsec_nanos() as f32 / 1.0e9;
        // move the camera
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

            if print_info {
                print_info = false;
                println!("{:?}, {:?}", camera.eye, camera.target);
            }
        }

    }
}