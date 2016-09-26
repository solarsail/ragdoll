#[derive(Debug)]
enum Terrain {
    Water = 0,
    Basin,
    Plain,
    Hill,
    Mountain,
}

enum Surface {
    Soil = 0,
    Grass,
    Forest,
    Sand,
    Snow,
    Ice,
}

struct TileSettings {
    radius: f32,
    surface_texture_ids: Vec<String>,
}