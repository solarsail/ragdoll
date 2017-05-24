pub mod game;
pub mod state;
pub mod states;
pub mod render;
pub mod input;

pub use self::input::InputHandler;
pub use self::game::Game;
pub use self::state::{StateMachine, State, Trans};

pub struct GameClock {
    pub dt: f32,
}

impl GameClock {
    pub fn new() -> GameClock {
        GameClock { dt: 0.0 }
    }
}
