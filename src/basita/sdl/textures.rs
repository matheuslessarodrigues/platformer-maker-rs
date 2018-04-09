use sdl2::image::LoadTexture;
use sdl2::render::{Texture, TextureCreator};
use sdl2::video::WindowContext;

use core::assets::AssetLoadError;

#[derive(Default)]
pub struct TextureStorage<'a> {
	textures: Vec<Texture<'a>>,
}

impl<'a> TextureStorage<'a> {
	pub fn add(&mut self, texture: Texture<'a>) -> usize {
		let index = self.textures.len();
		self.textures.push(texture);
		index
	}

	pub fn at(&self, index: usize) -> &Texture<'a> {
		&self.textures[index]
	}
}

pub struct TextureLoader {
	texture_creator: TextureCreator<WindowContext>,
}

impl TextureLoader {
	pub fn new(texture_creator: TextureCreator<WindowContext>) -> Self {
		TextureLoader {
			texture_creator: texture_creator,
		}
	}

	pub fn load_texture<'a>(
		&'a self,
		path: &str,
		storage: &mut TextureStorage<'a>,
	) -> Result<usize, AssetLoadError> {
		match self.texture_creator.load_texture(path) {
			Ok(texture) => Ok(storage.add(texture)),
			Err(message) => Err(AssetLoadError::new(message)),
		}
	}
}

//----

pub struct Textures<'a> {
	pub textures: Vec<Texture<'a>>,
	pub texture_creator: TextureCreator<WindowContext>,
}

impl<'a> Textures<'a> {
	pub fn new(texture_creator: TextureCreator<WindowContext>) -> Self {
		Textures {
			texture_creator: texture_creator,
			textures: Vec::new(),
		}
	}

	pub fn load_texture(&'a mut self, path: &str) -> Result<(&Texture<'a>, usize), AssetLoadError> {
		match self.texture_creator.load_texture(path) {
			Ok(texture) => {
				let index = self.textures.len();
				self.textures.push(texture);
				Ok((&self.textures[index], index))
			}
			Err(message) => Err(AssetLoadError::new(message)),
		}
	}

	pub fn at(&'a self, index: usize) -> &Texture<'a> {
		&self.textures[index]
	}
}
