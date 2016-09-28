use coordinates::Coordinates;
use tile::{Terrain, Surface};

pub struct Map {
    tiles: Vec<Coordinates>,
    terrains: Vec<Terrain>,
    surfaces: Vec<Surface>,
}

impl Map {
    /*
    fn load_from_file(name: &str) -> Self {

    }
    */
    pub fn new() -> Self {
        Map {
            tiles: vec![
                Coordinates::new(0, 0),
                Coordinates::new(1, 0),
                Coordinates::new(0, 1),
            ],
            terrains: vec![
                Terrain::Plain,
                Terrain::Hill,
                Terrain::Basin
            ],
            surfaces: vec![
                Surface::Grass,
                Surface::Sand,
                Surface::Snow
            ],
        }
    }

    pub fn size(&self) -> usize {
        self.tiles.len()
    }

    pub fn iter(&self) -> MapIterator {
        MapIterator {
            map: self,
            index: 0,
            size: self.size(),
        }
    }
}

pub struct MapIterator<'a> {
    map: &'a Map,
    index: usize,
    size: usize,
}

impl<'a> Iterator for MapIterator<'a> {
    type Item = (Coordinates, Terrain, Surface);

    fn next(&mut self) -> Option<Self::Item> {
        if self.index < self.size {
            let i = self.index;
            self.index += 1;
            Some((self.map.tiles[i], self.map.terrains[i], self.map.surfaces[i]))
        } else {
            None
        }
    }
}