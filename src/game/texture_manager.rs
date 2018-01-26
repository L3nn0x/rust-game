extern crate sld2;

use sdl2::render::{TextureCreator, Texture};

type TextureManager<'a, T> = ResourceManager<String, Texture<'a>, TextureCreator<T>>;

impl<'a, T> ResourceLoader<Texture<'a>> for TextureCreator<T> {
  type Args = str;
  fn load(&self, path: &str) -> Result<texture, String> {
    self.load_texture(path);
  }
}
