#[derive(Debug)]
pub struct Settings {
    pub scroll_rate: f32,
}

impl Settings {
    pub fn new() -> Settings {
        Settings {
            scroll_rate: 0.02,
        }
    }
}