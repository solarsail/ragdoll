use std::collections::HashMap;
use std::slice::Iter;
use sdl2::event::Event;
use sdl2::render::Renderer;
use game::GameContext;


pub trait GameState {
    fn on_update(&mut self, ctx: &mut GameContext, dfa: &mut StateMachine, dt: f64);
    fn on_input(&mut self, ctx: &mut GameContext, dfa: &mut StateMachine);
    fn on_render(&mut self, ctx: &mut GameContext, w: &mut Renderer);
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum StateTrans {
    Title,
    Gameplay,
    Pause,
    Resume,
}


pub struct StateMachine {
    trans: Vec<HashMap<StateTrans, usize>>,
    preserve: Vec<bool>,
    ui_stack: Vec<usize>,
    initial: usize,
    current: usize,
    state_changed: bool,
}

impl StateMachine {
    pub fn new() -> StateMachine {
        StateMachine {
            trans: Vec::new(),
            preserve: Vec::new(),
            ui_stack: Vec::new(),
            initial: 0,
            current: 0,
            state_changed: false,
        }
    }

    pub fn add_state(&mut self,
                     states: &mut Vec<Box<GameState>>,
                     s: Box<GameState>,
                     preserve: bool)
                     -> usize {
        states.push(s);
        self.preserve.push(preserve);
        self.trans.push(HashMap::new());
        states.len() - 1
    }

    pub fn add_trans(&mut self, from: usize, to: usize, action: StateTrans) {
        self.trans[from].insert(action, to);
    }

    pub fn set_initial(&mut self, i: usize) {
        self.initial = i;
        self.current = i;
        self.ui_stack.push(i);
    }

    pub fn feed(&mut self, action: StateTrans) {
        if let Some(&id) = self.trans[self.current].get(&action) {
            if !self.preserve[self.current] {
                self.ui_stack.pop();
            }
            self.current = id;
            self.ui_stack.push(id);
            self.state_changed = true;
        }
    }

    pub fn current_state_id(&self) -> usize {
        self.current
    }

    pub fn new_state(&mut self) -> Option<usize> {
        if self.state_changed {
            self.state_changed = false;
            Some(self.current)
        } else {
            None
        }
    }

    pub fn ui_stack(&self) -> Vec<usize> {
        self.ui_stack.clone()
    }
}
