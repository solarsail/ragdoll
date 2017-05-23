use std::collections::{HashSet, VecDeque};

use sdl2::event::Event;

pub use sdl2::keyboard::Keycode;
pub use sdl2::mouse::MouseButton;


#[derive(Debug)]
pub struct Click {
    pub btn: MouseButton,
    pub x: i32,
    pub y: i32,
}

pub struct InputHandler {
    keys_down: HashSet<Keycode>,
    mouse_down: HashSet<MouseButton>,
    mouse_clicked: VecDeque<Click>,
    cursor_pos: [i32; 2],
}

impl InputHandler {
    pub fn new() -> InputHandler {
        InputHandler {
            keys_down: HashSet::new(),
            mouse_down: HashSet::new(),
            mouse_clicked: VecDeque::new(),
            cursor_pos: [0, 0],
        }
    }

    pub fn update(&mut self, e: &Event) -> bool {
        let mut processed = true;
        match *e {
            Event::KeyDown { keycode: Some(c), .. } => {
                self.keys_down.insert(c);
                //debug!("key down: {:?}", c);
            }
            Event::KeyUp { keycode: Some(c), .. } => {
                self.keys_down.remove(&c);
                //debug!("key up: {:?}", c);
            }
            Event::MouseButtonDown { mouse_btn, x, y, .. } => {
                self.cursor_pos = [x, y];
                self.mouse_down.insert(mouse_btn);
                //debug!("mouse down: {:?}", mouse_btn);
            }
            Event::MouseButtonUp { mouse_btn, x, y, .. } => {
                self.cursor_pos = [x, y];
                //debug!("mouse up: {:?}", mouse_btn);
                if self.mouse_down.remove(&mouse_btn) {
                    self.mouse_clicked
                        .push_back(Click {
                                       btn: mouse_btn,
                                       x,
                                       y,
                                   });
                    //debug!("mouse click: {:?}", mouse_btn);
                }
            }
            Event::MouseMotion { x, y, .. } => {
                self.cursor_pos = [x, y];
                //debug!("mouse move: ({}, {})", x, y);
            }
            _ => {
                processed = false;
            }
        }

        processed
    }

    pub fn key_down(&self, k: Keycode) -> bool {
        self.keys_down.contains(&k)
    }

    pub fn keys_down(&self, ks: &[Keycode]) -> bool {
        ks.iter().all(|k| self.keys_down.contains(k))
    }

    pub fn clicked_iter(&mut self) -> ClickIter {
        ClickIter { q: &mut self.mouse_clicked }
    }

    pub fn mouse_pos(&self) -> &[i32; 2] {
        &self.cursor_pos
    }
}


pub struct ClickIter<'a> {
    q: &'a mut VecDeque<Click>,
}

impl<'a> Iterator for ClickIter<'a> {
    type Item = Click;
    fn next(&mut self) -> Option<Click> {
        self.q.pop_front()
    }
}
