use std::collections::HashMap;
use std::path::PathBuf;
use std::rc::Rc;
use std::borrow::Borrow;
use std::hash::Hash;

use sdl2::render::{Texture, TextureCreator, WindowCanvas};
use sdl2::video::WindowContext;
use sdl2::image::LoadTexture;
use sdl2::pixels::PixelFormatEnum;
use sdl2::ttf::{Font, Sdl2TtfContext};
use default;


type TextureManager<'l, T> = ResourceManager<'l, String, Texture<'l>, TextureCreator<T>>;
type FontManager<'l> = ResourceManager<'l, FontDetails, Font<'l, 'static>, Sdl2TtfContext>;

// Generic struct to cache any resource loaded by a ResourceLoader
pub struct ResourceManager<'l, K, R, L>
    where K: Hash + Eq,
          L: 'l + ResourceLoader<'l, R>
{
    loader: &'l L,
    cache: HashMap<K, Rc<R>>,
}

impl<'l, K, R, L> ResourceManager<'l, K, R, L>
    where K: Hash + Eq,
          L: ResourceLoader<'l, R>
{
    pub fn new(loader: &'l L) -> Self {
        ResourceManager {
            cache: HashMap::new(),
            loader: loader,
        }
    }

    // Generics magic to allow a HashMap to use String as a key
    // while allowing it to use &str for gets
    pub fn load<D>(&mut self, details: &D) -> Result<Rc<R>, String>
        where L: ResourceLoader<'l, R, Args = D>,
              D: Eq + Hash + ?Sized,
              K: Borrow<D> + for<'a> From<&'a D>
    {
        self.cache
            .get(details)
            .cloned()
            .map_or_else(|| {
                             let resource = Rc::new(self.loader.load(details)?);
                             self.cache.insert(details.into(), resource.clone());
                             Ok(resource)
                         },
                         Ok)
    }
}

// TextureCreator knows how to load Textures
impl<'l, T> ResourceLoader<'l, Texture<'l>> for TextureCreator<T> {
    type Args = str;
    fn load(&'l self, path: &str) -> Result<Texture, String> {
        //println!("LOADED A TEXTURE");
        if path == "NOT_FOUND" {
            warn!("texture not found");
            let mut not_found = self.create_texture_streaming(PixelFormatEnum::RGB24, 32, 32)
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

            Ok(not_found)
        } else {
            self.load_texture(path)
        }

    }
}

// Font Context knows how to load Fonts
impl<'l> ResourceLoader<'l, Font<'l, 'static>> for Sdl2TtfContext {
    type Args = FontDetails;
    fn load(&'l self, details: &FontDetails) -> Result<Font<'l, 'static>, String> {
        println!("LOADED A FONT");
        self.load_font(&details.path, details.size)
    }
}

// Generic trait to Load any Resource Kind
pub trait ResourceLoader<'l, R> {
    type Args: ?Sized;
    fn load(&'l self, data: &Self::Args) -> Result<R, String>;
}

// Information needed to load a Font
#[derive(PartialEq, Eq, Hash)]
pub struct FontDetails {
    pub path: String,
    pub size: u16,
}

impl<'a> From<&'a FontDetails> for FontDetails {
    fn from(details: &'a FontDetails) -> FontDetails {
        FontDetails {
            path: details.path.clone(),
            size: details.size,
        }
    }
}


pub struct AssetManager<'l> {
    texture_manager: TextureManager<'l, WindowContext>,
    font_manager: FontManager<'l>,
}

impl<'l> AssetManager<'l> {
    pub fn new(texture_creator: &'l TextureCreator<WindowContext>,
               ttf_ctx: &'l Sdl2TtfContext)
               -> AssetManager<'l> {
        AssetManager {
            texture_manager: TextureManager::new(texture_creator),
            font_manager: FontManager::new(ttf_ctx),
        }
    }

    pub fn texture(&mut self, id: &str) -> Rc<Texture<'l>> {
        self.texture_manager
            .load(id)
            .unwrap_or(self.texture_manager.load("NOT_FOUND").unwrap())
    }
}


/*
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
*/
