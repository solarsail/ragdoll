extern crate piston_window;

use piston_window::*;
use default;


pub struct Resources {
    font: Option<Glyphs>,
    logo_texture: G2dTexture,
}

impl Resources {
    pub fn new(window: &mut PistonWindow) -> Self {
        let img_path = default::assets_path().join("images").join("rust-logo.png");
        Resources {
            font: Glyphs::new(
                default::assets_path().join("fonts").join("RussoOne-Regular.ttf"),
                window.factory.clone()).ok(),
            logo_texture: Texture::from_path(
                &mut window.factory,
                &img_path,
                Flip::None,
                &TextureSettings::new()).unwrap(),
        }
    }

    pub fn font(&mut self) -> &mut Glyphs {
        self.font.as_mut().unwrap()
    }

    pub fn logo_texture(&self) -> &G2dTexture {
        &self.logo_texture
    }
}
