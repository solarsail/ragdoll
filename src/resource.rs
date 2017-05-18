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


type TextureManager<'l, T> = ResourceManager<'l, String, Texture<'l>, TextureCreator<T>>;
type FontManager<'l> = ResourceManager<'l, FontDetails, Font<'l, 'static>, Sdl2TtfContext>;

/// 通用资源管理器，缓存从资源加载器中获取的资源。
pub struct ResourceManager<'loader, K, R, L>
    where K: Default,
          L: 'loader + ResourceLoader<'loader, R>
{
    loader: &'loader L,
    cache: HashMap<String, Rc<RefCell<R>>>,
    id_mapping: HashMap<String, K>,
}

impl<'loader, K, R, L> ResourceManager<'loader, K, R, L>
    where K: Default,
          L: ResourceLoader<'loader, R>
{
    pub fn new<M>(loader: &'loader L, mapper: &M, mapping_file: PathBuf) -> Self
        where M: IdMapper
    {
        let id_mapping = mapper.load_mapping(mapping_file);
        ResourceManager {
            loader: loader,
            cache: HashMap::new(),
            id_mapping,
        }
    }

    pub fn load<D>(&mut self, id: D) -> Result<Rc<RefCell<R>>, String>
        where D: Into<String>
    {
        self.cache
            .get(details)
            .cloned()
            .map_or_else(|| {
                let details = self.id_mapping
                    .get(id.into())
                    .unwrap_or_else(|| {
                                        warn!("Texture not found: {}", &id.into());
                                        K::default()
                                    });
                let resource = Rc::new(RefCell::new(self.loader.load(details)?));
                self.cache.insert(id.into(), resource.clone());
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
        if path == String::default() {
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

/// 加载字体所需的信息。
#[derive(PartialEq, Eq, Hash, Deserialize)]
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
            texture_manager: TextureManager::new(texture_creator),
            font_manager: FontManager::new(ttf_ctx),
        }
    }

    pub fn texture(&mut self, id: &str) -> Rc<RefCell<Texture<'l>>> {
        self.texture_manager.load(id).unwrap()
    }
}


/*

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
