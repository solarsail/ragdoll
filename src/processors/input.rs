use std::sync::{Mutex, Arc};
use std::collections::VecDeque;

use amethyst::context::Context;
use amethyst::ecs::{Join, Processor, RunArg};

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

#[derive(Debug)]
pub enum InputEvent {
    MouseLeftClicked(i32, i32),
    MouseRightClicked(i32, i32),
    PrintCameraInfo,
}

pub struct InputEvents {
    pub camera: VecDeque<InputEvent>,
    pub unit: VecDeque<InputEvent>,
}

impl InputEvents {
    pub fn new() -> Self {
        InputEvents {
            camera: VecDeque::with_capacity(10),
            unit: VecDeque::with_capacity(10),
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
        let (mut input_state,
             mut input_events) = arg.fetch(|w| (w.write_resource::<InputState>(),
                                                w.write_resource::<InputEvents>()));

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

                Event::KeyboardInput(ElementState::Released, _, Some(VirtualKeyCode::P)) => {
                    input_events.camera.push_back(InputEvent::PrintCameraInfo);
                }

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
                        let (x, y) = input_state.cursor_pos;
                        input_events.unit.push_back(InputEvent::MouseLeftClicked(x, y));
                    }
                }
                Event::MouseInput(ElementState::Released, MouseButton::Right) => {
                    input_state.rmb = false;
                    if input_state.cursor_pos == input_state.rmb_pos {
                        let (x, y) = input_state.cursor_pos;
                        input_events.unit.push_back(InputEvent::MouseRightClicked(x, y));
                    }
                }
                
                _ => (),
            }
        }


        //let delta_time = context.delta_time.subsec_nanos() as f32 / 1.0e9;
    }
}