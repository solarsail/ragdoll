use std::collections::hash_map::{HashMap, Entry};
use std::path::PathBuf;
use std::marker::PhantomData;

use sdl2::render::{Texture, TextureCreator, WindowCanvas};
use sdl2::video::WindowContext;
use sdl2::image::LoadTexture;
use sdl2::pixels::PixelFormatEnum;
use sdl2::ttf::{Font, Sdl2TtfContext};
use default;


struct TextureLoader<'r> {
    creator: TextureCreator<WindowContext>,
    conf: HashMap<String, PathBuf>,
    _marker: PhantomData<&'r ()>,
}

impl<'r> TextureLoader<'r> {
    fn new(creator: TextureCreator<WindowContext>, conf_file: &str) -> TextureLoader {
        let mut conf = HashMap::new();
        // TODO: load conf file
        // debug
        let image_path = default::assets_path()
            .join("images")
            .join("rust-logo.png");
        let rect_path = default::assets_path().join("images").join("rect.png");
        conf.insert("logo".into(), image_path);
        conf.insert("tile".into(), rect_path);
        // debug end
        TextureLoader {
            creator,
            conf,
            _marker: PhantomData,
        }
    }

    fn load<T: Into<String>>(&'r self, id: T) -> Option<Texture> {
        self.creator
            .load_texture(self.conf.get(&id.into()).unwrap())
            .map_err(|e| {
                         warn!("unable to load texture: {}", e);
                         e
                     })
            .ok()
    }

    fn not_found(&'r self) -> Texture {
        let mut not_found = self.creator
            .create_texture_streaming(PixelFormatEnum::RGB24, 32, 32)
            .unwrap();
        // Create a red-green gradient
        not_found
            .with_lock(None, |buffer: &mut [u8], pitch: usize| for y in 0..32 {
                for x in 0..32 {
                    let offset = y * pitch + x * 3;
                    buffer[offset] = x as u8;
                    buffer[offset + 1] = y as u8;
                    buffer[offset + 2] = 0;
                }
            })
            .unwrap();

        not_found
    }
}


pub struct AssetManager<'ttf, 'r> {
    // TODO: FontLoader
    title_font: Font<'ttf, 'static>,
    caption_font: Font<'ttf, 'static>,
    textures: HashMap<String, Texture<'r>>,
    loader: TextureLoader<'r>,
}

impl<'ttf, 'r> AssetManager<'ttf, 'r> {
    pub fn new(ttf_ctx: &'ttf Sdl2TtfContext, canvas: &WindowCanvas) -> AssetManager<'ttf, 'r> {
        let font_path = default::assets_path()
            .join("fonts")
            .join("RussoOne-Regular.ttf");

        let texture_creator = canvas.texture_creator();
        AssetManager {
            title_font: ttf_ctx.load_font(&font_path, 30).unwrap(),
            caption_font: ttf_ctx.load_font(&font_path, 22).unwrap(),
            textures: HashMap::new(),
            loader: TextureLoader::new(texture_creator, "fake_conf"),
        }
    }

    pub fn title_font(&self) -> &Font {
        &self.title_font
    }

    pub fn caption_font(&self) -> &Font {
        &self.caption_font
    }

    pub fn texture<T: Into<String>>(&'r mut self, id: T) -> &Texture {
        let id = id.into();
        self.textures
            .entry(id.clone())
            .or_insert(self.loader.load(id).unwrap_or(self.loader.not_found()))
    }
}
