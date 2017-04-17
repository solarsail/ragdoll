use sdl2::render::Renderer;
use sdl2::render::Texture;
use sdl2::image::LoadTexture;
use sdl2::ttf::{Font, Sdl2TtfContext};
use default;


pub struct Resources<'ttf> {
    title_font: Font<'ttf, 'static>,
    caption_font: Font<'ttf, 'static>,
    logo_texture: Texture,
}

impl<'ttf> Resources<'ttf> {
    pub fn new(ttf_ctx: &'ttf Sdl2TtfContext, renderer: &mut Renderer) -> Self {
        let image_path = default::assets_path().join("images").join("rust-logo.png");
        let font_path = default::assets_path().join("fonts").join("RussoOne-Regular.ttf");
        
        Resources {
            title_font: ttf_ctx.load_font(&font_path, 30).unwrap(),
            caption_font: ttf_ctx.load_font(&font_path, 22).unwrap(),
            logo_texture: renderer.load_texture(&image_path).unwrap(),
        }
    }

    pub fn title_font(&self) -> &Font {
        &self.title_font
    }

    pub fn caption_font(&self) -> &Font {
        &self.caption_font
    }

    pub fn logo_texture(&self) -> &Texture {
        &self.logo_texture
    }
}
