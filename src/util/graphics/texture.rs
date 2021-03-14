use macroquad::prelude::FilterMode::Nearest;
use macroquad::prelude::set_texture_filter;
use super::Texture;

pub async fn load_texture<P: AsRef<std::path::Path>>(path: P) -> Texture {
	let path = path.as_ref();
	let texture = macroquad::prelude::load_texture(path.to_str().expect("Could not unwrap path to string")).await;
	set_texture_filter(texture, Nearest);
	texture
}

pub fn byte_texture(bytes: &[u8]) -> Texture { // To - do: change to Result<Texture, ImageError>
	image_texture(&crate::util::image::byte_image(bytes).expect("Could not get bytes from image!"))
}

pub fn image_texture(image: &macroquad::prelude::Image) -> Texture {
	let texture = macroquad::prelude::load_texture_from_image(image);
	set_texture_filter(texture, Nearest);
	texture
}

pub fn debug_texture() -> Texture {
	byte_texture(include_bytes!("../../../build/assets/missing_texture.png"))
}