use std::path::Path;
use image::{RgbaImage, SubImage};
use crate::util::render_util::TEXTURE_SIZE;

pub fn open_image<P>(path: P) -> Option<RgbaImage> where P: AsRef<Path> {
    let path = path.as_ref();
    match image::open(path) {
        Ok(img) => {
            let img = match img {
                image::DynamicImage::ImageRgba8(img) => img,
                other => other.to_rgba8(),
            };
            Some(img)
        },
        Err(err) => {
            log::warn!("Could not load image at {:?} with error: {}", path.file_name().unwrap(), err);
            None
        }
    }
}

pub fn get_subimage(image: &RgbaImage, id: usize) -> RgbaImage {
    return get_subimage_wh(image, id, TEXTURE_SIZE as u32, TEXTURE_SIZE as u32);
}

pub fn get_subimage_wh(image: &RgbaImage, id: usize, w: u32, h: u32) -> RgbaImage {
	let x: u32;
	let y: u32;
	x = id as u32 % (image.width() / w as u32);
	y = id as u32 / (image.width() / w as u32);
	return get_subimage_at(image, x * w, y * h, w, h);
}

pub fn get_subimage_at(image: &RgbaImage, x: u32, y: u32, w: u32, h: u32) -> RgbaImage {
    return SubImage::new(image, x, y, w as u32, h as u32).to_image();
}