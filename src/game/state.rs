use sdl2::event::Event;
use specs::Planner;
use resource::AssetManager;


/// Types of state transitions.
pub enum Trans {
    /// Continue as normal.
    None,
    /// Remove the active state and resume the next state on the stack or stop
    /// if there are none.
    Pop,
    /// Pause the active state and push a new state onto the stack.
    Push(Box<State>),
    /// Remove the current state on the stack and insert a different one.
    Switch(Box<State>),
    /// Stop and remove all states and shut down the engine.
    Quit,
}

/// A trait which defines game states that can be used by the state machine.
pub trait State {
    /// Executed when the game state begins.
    fn on_start(&mut self, _planner: &mut Planner<()>, _assets: &mut AssetManager) {}

    /// Executed when the game state exits.
    fn on_stop(&mut self, _planner: &mut Planner<()>, _assets: &mut AssetManager) {}

    /// Executed when a different game state is pushed onto the stack.
    fn on_pause(&mut self, _planner: &mut Planner<()>, _assets: &mut AssetManager) {}

    /// Executed when the application returns to this game state once again.
    fn on_resume(&mut self, _planner: &mut Planner<()>, _assets: &mut AssetManager) {}

    /// Executed on every frame before updating, for use in reacting to events.
    fn handle_event(&mut self,
                    _event: &Event,
                    _planner: &mut Planner<()>,
                    _assets: &mut AssetManager)
                    -> Trans {
        Trans::None
    }

    /// Executed on every frame immediately, as fast as the engine will allow.
    fn update(&mut self,
              _planner: &mut Planner<()>,
              _assets: &mut AssetManager,
              _dt: f32)
              -> Trans {
        Trans::None
    }
}

pub struct StateMachine {
    running: bool,
    state_stack: Vec<Box<State>>,
}

/// 下推自动机，用于管理游戏状态。
/// 游戏状态以栈符号形式存在，与自动机本身的状态集无关。
/// 自动机只有两个状态：运行和停止。
impl StateMachine {
    /// Creates a new state machine with the given initial state.
    pub fn new<T>(initial_state: T) -> StateMachine
        where T: State + 'static
    {
        StateMachine {
            running: false,
            state_stack: vec![Box::new(initial_state)],
        }
    }

    /// Checks whether the state machine is running.
    pub fn is_running(&self) -> bool {
        self.running
    }

    /// Initializes the state machine.
    ///
    /// # Panics
    /// Panics if no states are present in the stack.
    pub fn start(&mut self, _planner: &mut Planner<()>, assets: &mut AssetManager) {
        if !self.running {
            let state = self.state_stack.last_mut().unwrap();
            state.on_start(_planner, assets);
            self.running = true;
        }
    }

    /// Passes a vector of events to the active state to handle.
    pub fn handle_event(&mut self,
                        event: &Event,
                        planner: &mut Planner<()>,
                        assets: &mut AssetManager) {
        if self.running {
            let trans = match self.state_stack.last_mut() {
                Some(state) => state.handle_event(event, planner, assets),
                None => Trans::None,
            };

            self.transition(trans, planner, assets);
        }
    }

    /// Updates the currently active state immediately.
    pub fn update(&mut self, planner: &mut Planner<()>, assets: &mut AssetManager, dt: f32) {
        if self.running {
            let trans = match self.state_stack.last_mut() {
                Some(state) => state.update(planner, assets, dt),
                None => Trans::None,
            };

            self.transition(trans, planner, assets);
        }
    }

    /// Performs a state transition, if requested by either update() or
    /// handle_event().
    fn transition(&mut self,
                  request: Trans,
                  _planner: &mut Planner<()>,
                  assets: &mut AssetManager) {
        if self.running {
            match request {
                Trans::None => (),
                Trans::Pop => self.pop(_planner, assets),
                Trans::Push(state) => self.push(state, _planner, assets),
                Trans::Switch(state) => self.switch(state, _planner, assets),
                Trans::Quit => self.stop(_planner, assets),
            }
        }
    }

    /// Removes the current state on the stack and inserts a different one.
    fn switch(&mut self,
              state: Box<State>,
              _planner: &mut Planner<()>,
              assets: &mut AssetManager) {
        if self.running {
            if let Some(mut state) = self.state_stack.pop() {
                state.on_stop(_planner, assets);
            }

            self.state_stack.push(state);
            let state = self.state_stack.last_mut().unwrap();
            state.on_start(_planner, assets);
        }
    }

    /// Pauses the active state and pushes a new state onto the state stack.
    fn push(&mut self, state: Box<State>, _planner: &mut Planner<()>, assets: &mut AssetManager) {
        if self.running {
            if let Some(state) = self.state_stack.last_mut() {
                state.on_pause(_planner, assets);
            }

            self.state_stack.push(state);
            let state = self.state_stack.last_mut().unwrap();
            state.on_start(_planner, assets);
        }
    }

    /// Stops and removes the active state and un-pauses the next state on the
    /// stack (if any).
    fn pop(&mut self, _planner: &mut Planner<()>, assets: &mut AssetManager) {
        if self.running {
            if let Some(mut state) = self.state_stack.pop() {
                state.on_stop(_planner, assets);
            }

            if let Some(mut state) = self.state_stack.last_mut() {
                state.on_resume(_planner, assets);
            } else {
                self.running = false;
            }
        }
    }

    /// Shuts the state machine down.
    fn stop(&mut self, _planner: &mut Planner<()>, assets: &mut AssetManager) {
        if self.running {
            while let Some(mut state) = self.state_stack.pop() {
                state.on_stop(_planner, assets);
            }

            self.running = false;
        }
    }
}
