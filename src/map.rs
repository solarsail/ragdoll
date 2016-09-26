use amethyst::ecs::{VecStorage, Component};
use coordinates::Coordinates;
use tilesettings::{Terrain, Surface};

#[derive(Debug)]
struct Map {
    tiles: Vec<Coordinates>,
    terrains: Vec<Terrain>,
    surfaces: Vec<Surface>,
}

impl Component for Map {
    type Storage = VecStorage<Map>;
}

impl Map {
    fn load_from_file(name: &str) -> Self {

    }
}