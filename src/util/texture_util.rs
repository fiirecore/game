use std::path::Path;
use opengl_graphics::{Filter, Texture, TextureSettings};

use crate::util::file_util::asset_as_pathbuf;

pub fn texture_from_path<P>(path: P) -> Texture where P: AsRef<Path> {

	let path = path.as_ref();

//	let result = Image::new(ctx, path);
	let result = Texture::from_path(Path::new(path), &TextureSettings::new().min(Filter::Nearest).mag(Filter::Nearest));
	
	match result {
		Ok(result) => {
			return result;
		}
		Err(result) => {
			println!("Error fetching texture with path: {:?}, Result: {}", path, result);
//			return Image::new(ctx, asset_as_pathbuf("debug/a_texture.png")).expect("Cannot find debug texture");
			Texture::from_path(asset_as_pathbuf("debug/missing_texture.png"), &TextureSettings::new().min(Filter::Nearest).mag(Filter::Nearest)).unwrap()
		}
	}
	
}

pub fn texture64_from_path<P>(path: P) -> Texture where P: AsRef<Path> {

	let path = path.as_ref();

//	let result = Image::new(ctx, path);
	let result = Texture::from_path(Path::new(path), &TextureSettings::new().min(Filter::Nearest).mag(Filter::Nearest));
	
	match result {
		Ok(result) => {
			return result;
		}
		Err(result) => {
			println!("Error fetching texture with path: {:?}, Result: {}", path, result);
//			return Image::new(ctx, asset_as_pathbuf("debug/a_texture.png")).expect("Cannot find debug texture");
			Texture::from_path(asset_as_pathbuf("debug/missing_texture_64.png"), &TextureSettings::new().min(Filter::Nearest).mag(Filter::Nearest)).unwrap()
		}
	}
	
}

/*

pub trait AnimatedTexture {

	fn update(&mut self, args: UpdateArgs);

	fn get(&mut self) -> &Texture;

	fn next(&mut self);

}

pub struct ConstantTimeTexture {
	
	time_since: usize,
	every: usize,
	
	textures: Vec<Texture>,
	position: usize,
	
}

impl ConstantTimeTexture {
	
	pub fn new(_textures: Vec<Texture>, _every: usize) -> ConstantTimeTexture {
		
		ConstantTimeTexture {
			
			time_since: 0,
			every: _every,
			
			textures: _textures,
			position: 0,
		}
		
	}
	
}

impl AnimatedTexture for ConstantTimeTexture {
	
	fn update(&mut self, _args: UpdateArgs) {
		if self.time_since < self.every {
			self.time_since += 1;
		} else {
			self.time_since = 0;
			self.next();
		}

	}
	
	fn get(&mut self) -> &Texture {
		&self.textures[self.position]
	}
	
	fn next(&mut self) {
		self.position = (self.position + 1) % (self.textures.len() - 1);
	}
	
}

*/