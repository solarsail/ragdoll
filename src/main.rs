extern crate sdl2;
extern crate hexgrid;
extern crate find_folder;

mod default;
mod map;
mod game;
mod view;
mod settings;
mod resource;

use game::states::{OpeningState, GamePlayState, TitleState, PauseState};
use game::{StateTrans, StateMachine, Game, GameState};
use settings::*;
use map::*;
use hexgrid::{Layout, POINTY_TOP};

fn main() {
    let settings = Settings::load("settings.ini");
    let wsize = [settings.window_width, settings.window_height];


    let mut states: Vec<Box<GameState>> = Vec::new();
    let mut dfa = StateMachine::new();

    let opening = dfa.add_state(&mut states, Box::new(OpeningState::new(4.0, 200, 200)), false);
    let title = dfa.add_state(&mut states, Box::new(TitleState::new()), false);
    let pause = dfa.add_state(&mut states, Box::new(PauseState::new()), false);
    let map = HexMap::new(5);
    let layout = Layout::new(POINTY_TOP, [20.0, 20.0], [200.0, 200.0]);
    let gameplay = dfa.add_state(&mut states, Box::new(GamePlayState::new(map, layout)), true);

    dfa.set_initial(opening);

    dfa.add_trans(opening, title, StateTrans::Title);
    dfa.add_trans(title, gameplay, StateTrans::Gameplay);
    dfa.add_trans(gameplay, pause, StateTrans::Pause);
    dfa.add_trans(pause, gameplay, StateTrans::Resume);

    let mut game = Game::new(settings, "title", &mut dfa, &mut states);

    game.run();
}
