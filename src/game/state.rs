extern crate piston_window;

use std::collections::HashMap;
use piston_window::*;
use game::{State, GameContext};


pub trait GameState {
    fn on_update(&mut self, gc: &mut GameContext, dt: f64);
    fn on_input(&mut self, gc: &mut GameContext, input: &Input);
    fn on_render(&mut self, gc: &mut GameContext, e: &Input, w: &mut PistonWindow);
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum StateTrans {
	Title,
    Gameplay,
    Pause,
    Resume,
}

pub struct StateMachine {
    states: Vec<Box<GameState>>,
    trans: Vec<HashMap<StateTrans, usize>>,
    initial: usize,
    current: usize,
    state_changed,
}

impl StateMachine {
	pub fn new() -> StateMachine {
		states: Vec::new(),
		trans: Vec::new(),
	}

	pub fn add_state(&mut self, s: Box<GameState>) -> usize {
		self.states.push(s);
		self.trans.push(HashMap::new());
		self.states.len() - 1
	}

	pub fn add_trans(&mut self, from: usize, to: usize, action: StateTrans) {
		self.trans[from].insert(action, to);
	}

	pub fn feed(&mut self, action: StateTrans) {
		if let Some(id) = self.trans[self.current].get(action) {
			self.current = id;
			self.state_changed = true;
		}
	}

	pub fn current_state(&self) -> Box<GameState> {
		self.states[self.current]
	}

	pub fn new_state(&self) -> Option<Box<GameState>> {
		if self.state_changed {
			self.state_changed = false;
			Some(states[self.current])
		} else {
			None
		}
	}
}