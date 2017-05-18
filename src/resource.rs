use std::collections::HashMap;
use std::path::PathBuf;
use std::rc::Rc;
use std::cell::RefCell;
use std::borrow::Borrow;
use std::hash::Hash;

use sdl2::render::{BlendMode, Texture, TextureCreator, WindowCanvas};
use sdl2::video::WindowContext;
use sdl2::image::LoadTexture;
use sdl2::pixels::PixelFormatEnum;
use sdl2::ttf::{Font, Sdl2TtfContext};
use default;


type TextureManager<'l, T> = ResourceManager<'l, PathBuf, Texture<'l>, TextureCreator<T>>;
type FontManager<'l> = ResourceManager<'l, FontDetails, Font<'l, 'static>, Sdl2TtfContext>;

/// 通用资源管理器，缓存从资源加载器中获取的资源。
pub struct ResourceManager<'loader, K, R, L>
    where K: Invalid,
          L: 'loader + ResourceLoader<'loader, R> + IdMapper<K>
{
    loader: &'loader L,
    cache: HashMap<String, Rc<RefCell<R>>>,
    id_mapping: HashMap<String, K>,
}

impl<'loader, K, R, L> ResourceManager<'loader, K, R, L>
    where K: Invalid,
          L: ResourceLoader<'loader, R> + IdMapper<K>
{
    pub fn new(loader: &'loader L, mapping_file: PathBuf) -> Self {
        let id_mapping = loader.mapping(mapping_file);
        ResourceManager {
            loader: loader,
            cache: HashMap::new(),
            id_mapping,
        }
    }

    pub fn load<D>(&mut self, id: D) -> Result<Rc<RefCell<R>>, String>
        where D: Into<String>,
              L: ResourceLoader<'loader, R, Args = K>
    {
        let not_found = K::invalid();
        let key = id.into();
        self.cache
            .get(&key)
            .cloned()
            .map_or_else(|| {
                let details = self.id_mapping
                    .get(&key)
                    .unwrap_or_else(|| {
                                        warn!("Texture not found: {}", &key);
                                        &not_found
                                    });
                let resource = Rc::new(RefCell::new(self.loader.load(details)?));
                self.cache.insert(key, resource.clone());
                Ok(resource)
            },
                         Ok)
    }
}

// TextureCreator knows how to load Textures
impl<'l, T> ResourceLoader<'l, Texture<'l>> for TextureCreator<T> {
    type Args = PathBuf;
    fn load(&'l self, path: &PathBuf) -> Result<Texture, String> {
        //println!("LOADED A TEXTURE");
        if *path == PathBuf::invalid() {
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
            not_found.set_blend_mode(BlendMode::Blend);
            Ok(not_found)
        } else {
            let mut texture = self.load_texture(path)?;
            texture.set_blend_mode(BlendMode::Blend);
            Ok(texture)
        }

    }
}

impl<T> IdMapper<PathBuf> for TextureCreator<T> {
    fn mapping(&self, mapping_file: PathBuf) -> HashMap<String, PathBuf> {
        let mut m = HashMap::new();
        // DEBUG
        let image_path = default::assets_path().join("images");
        m.insert("logo".into(), image_path.join("rust-logo.png"));
        m
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

impl IdMapper<FontDetails> for Sdl2TtfContext {
    fn mapping(&self, mapping_file: PathBuf) -> HashMap<String, FontDetails> {
        let mut m = HashMap::new();
        // DEBUG
        m
    }
}


impl Invalid for PathBuf {
    fn invalid() -> PathBuf {
        default::assets_path().join("invalid")
    }
}

impl Invalid for FontDetails {
    fn invalid() -> FontDetails {
        FontDetails {
            path: PathBuf::invalid(),
            size: 0,
        }
    }
}


// Generic trait to Load any Resource Kind
pub trait ResourceLoader<'l, R> {
    type Args: ?Sized;
    fn load(&'l self, data: &Self::Args) -> Result<R, String>;
}

pub trait IdMapper<D> {
    fn mapping(&self, mapping_file: PathBuf) -> HashMap<String, D>;
}

pub trait Invalid {
    fn invalid() -> Self;
}

/// 加载字体所需的信息。
#[derive(PartialEq, Eq, Hash)]
pub struct FontDetails {
    pub path: PathBuf,
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


/// 资源管理器，包括纹理和字体管理，以及资源文件与id的对应关系。
pub struct AssetManager<'l> {
    texture_manager: TextureManager<'l, WindowContext>,
    font_manager: FontManager<'l>,
}

impl<'l> AssetManager<'l> {
    pub fn new(texture_creator: &'l TextureCreator<WindowContext>,
               ttf_ctx: &'l Sdl2TtfContext)
               -> AssetManager<'l> {
        AssetManager {
            texture_manager: TextureManager::new(texture_creator, "fake_path".into()),
            font_manager: FontManager::new(ttf_ctx, "fake_path".into()),
        }
    }

    pub fn texture(&mut self, id: &str) -> Rc<RefCell<Texture<'l>>> {
        self.texture_manager.load(id).unwrap()
    }
}


/*


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
