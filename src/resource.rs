use std::collections::HashMap;
use std::path::PathBuf;
use std::rc::Rc;
use std::cell::RefCell;
use std::borrow::Borrow;
use std::hash::Hash;
use std::fs::File;
use std::io::prelude::*;

use sdl2::render::{BlendMode, Texture, TextureCreator, WindowCanvas};
use sdl2::video::WindowContext;
use sdl2::image::LoadTexture;
use sdl2::pixels::{PixelFormatEnum, Color};
use sdl2::ttf::{Font, Sdl2TtfContext};
use serde_yaml;
use default;


type TextureManager<'l, T> = ResourceManager<'l, PathBuf, Texture<'l>, TextureCreator<T>>;
type FontManager<'l> = ResourceManager<'l, FontDetails, Font<'l, 'static>, Sdl2TtfContext>;

/// 通用资源管理器，缓存从资源加载器中获取的资源。
pub struct ResourceManager<'loader, K, R, L>
    where L: 'loader + ResourceLoader<'loader, R> + IdMapper<K>
{
    pub loader: &'loader L,
    cache: HashMap<String, Rc<RefCell<R>>>,
    id_mapping: HashMap<String, K>,
}

impl<'l, T> ResourceManager<'l, PathBuf, Texture<'l>, TextureCreator<T>> {
    pub fn insert(&mut self, id: &str, t: Texture<'l>) {
        let r = Rc::new(RefCell::new(t));
        self.cache.insert(id.into(), r);
    }
}

impl<'loader, K, R, L> ResourceManager<'loader, K, R, L>
    where L: ResourceLoader<'loader, R> + IdMapper<K>
{
    pub fn new(loader: &'loader L) -> Self {
        let id_mapping = loader.mapping();
        ResourceManager {
            loader: loader,
            cache: HashMap::new(),
            id_mapping,
        }
    }

    pub fn load(&mut self, id: &str) -> Result<Rc<RefCell<R>>, String>
        where L: ResourceLoader<'loader, R, Args = K>
    {
        let key = id.into();
        self.cache
            .get(&key)
            .cloned()
            .map_or_else(|| {
                             let details = self.id_mapping.get(&key).ok_or("texture not found")?;
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
        let mut texture = self.load_texture(path)?;
        texture.set_blend_mode(BlendMode::Blend);
        Ok(texture)

    }
}

impl<T> IdMapper<PathBuf> for TextureCreator<T> {
    fn mapping(&self) -> HashMap<String, PathBuf> {
        let root = default::assets_path().join("images");
        let mapping_file = root.join("mapping.yaml");
        let f = File::open(mapping_file).unwrap();
        let mut m: HashMap<String, PathBuf> = serde_yaml::from_reader(f).unwrap();
        for (_, val) in m.iter_mut() {
            *val = root.join(&val);
        }
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
    fn mapping(&self) -> HashMap<String, FontDetails> {
        let root = default::assets_path().join("fonts");
        let mapping_file = root.join("mapping.yaml");
        let f = File::open(mapping_file).unwrap();
        let mut m: HashMap<String, FontDetails> = serde_yaml::from_reader(f).unwrap();
        for (_, val) in m.iter_mut() {
            val.path = root.join(&val.path);
        }
        m
    }
}


// Generic trait to Load any Resource Kind
pub trait ResourceLoader<'l, R> {
    type Args: ?Sized;
    fn load(&'l self, data: &Self::Args) -> Result<R, String>;
}

pub trait IdMapper<D> {
    fn mapping(&self) -> HashMap<String, D>;
}


/// 加载字体所需的信息。
#[derive(PartialEq, Eq, Hash, Deserialize)]
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
    texture_not_found: Rc<RefCell<Texture<'l>>>,
}

impl<'l> AssetManager<'l> {
    pub fn new(texture_creator: &'l TextureCreator<WindowContext>,
               ttf_ctx: &'l Sdl2TtfContext)
               -> AssetManager<'l> {
        let mut not_found = texture_creator
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
        not_found.set_blend_mode(BlendMode::Blend);

        AssetManager {
            texture_manager: TextureManager::new(texture_creator),
            font_manager: FontManager::new(ttf_ctx),
            texture_not_found: Rc::new(RefCell::new(not_found)),
        }
    }

    pub fn texture(&mut self, id: &str) -> Rc<RefCell<Texture<'l>>> {
        self.texture_manager
            .load(id)
            .unwrap_or(self.texture_not_found.clone())
    }

    pub fn font(&mut self, id: &str, text: &str, color: Color) -> Rc<RefCell<Texture<'l>>> {
        let font_texture_id = format!("font_{}", id);
        if let Ok(t) = self.texture_manager.load(&font_texture_id) {
            t
        } else if let Ok(f) = self.font_manager.load(id) {
            let surface = f.as_ref()
                .borrow()
                .render(text)
                .blended(color)
                .unwrap();
            let texture = self.texture_manager
                .loader
                .create_texture_from_surface(&surface)
                .unwrap();
            self.texture_manager.insert(&font_texture_id, texture);
            self.texture_manager.load(&font_texture_id).unwrap()
        } else {
            self.texture_not_found.clone()
        }

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
