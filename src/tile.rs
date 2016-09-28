#[derive(Debug, Clone, Copy)]
pub enum Terrain {
    Water = 0,
    Basin,
    Plain,
    Hill,
    Plateau,
    // Last
    Total,
}

#[derive(Debug, Clone, Copy)]
pub enum Surface {
    Soil = 0,
    Grass,
    Forest,
    Sand,
    Snow,
    Ice,
    // Last
    Total,
}

pub struct TileSettings {
    radius: f32,
    surface_texture_ids: Vec<String>,
    terrain_mesh_ids: Vec<String>,
}

impl TileSettings {
    pub fn new(radius: f32) -> Self {
        TileSettings {
            radius: radius,
            surface_texture_ids: vec!["invalid".to_string(); Surface::Total as usize],
            terrain_mesh_ids: vec!["invalid".to_string(); Terrain::Total as usize],
        }
    }

    pub fn set_terrain_mesh(&mut self, t: Terrain, m: String) {
        self.terrain_mesh_ids[t as usize] = m;
    }

    pub fn get_terrain_mesh(&self, t: Terrain) -> &String {
        &self.terrain_mesh_ids[t as usize]
    }

    pub fn set_surface_texture(&mut self, s: Surface, t: String) {
        self.surface_texture_ids[s as usize] = t;
    }

    pub fn get_surface_texture(&self, s: Surface) -> &String {
        &self.surface_texture_ids[s as usize]
    }

    pub fn radius(&self) -> f32 {
        self.radius
    }
}