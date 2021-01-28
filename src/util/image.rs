use image::RgbaImage;
use macroquad::prelude::Image;
use crate::util::render::TEXTURE_SIZE;

pub fn open_image_bytes(bytes: &[u8]) -> Image {
    Image::from_file_with_format(bytes, Some(image::ImageFormat::PNG))
}

pub async fn open_image<P: AsRef<std::path::Path>>(path: P) -> Option<Image> {
    let path = path.as_ref();
    if path.exists() {
        Some(macroquad::prelude::load_image(path.to_str().unwrap()).await)
    } else {
        None
    }
    
}

pub fn open_image_noasync<P: AsRef<std::path::Path>>(path: P) -> Option<Image> {
    let path = path.as_ref();
    match super::file::read_noasync(path) {
        Some(bytes) => Some(open_image_bytes(bytes.as_slice())),
        None => {
            macroquad::prelude::warn!("Could not read image bytes at {:?} with error", path);
            return None;
        }
    }
}

pub fn get_subimage(image: &Image, id: usize) -> Image {
    return get_subimage_wh(image, id, TEXTURE_SIZE as u32, TEXTURE_SIZE as u32);
}

pub fn get_subimage_wh(image: &Image, id: usize, w: u32, h: u32) -> Image {
	let x = id % (image.width() / w as usize);
	let y = id / (image.width() / w as usize);
	return get_subimage_at(image, x as u32 * w, y as u32 * h, w, h);
}

pub fn get_subimage_at(image: &Image, x: u32, y: u32, w: u32, h: u32) -> Image {
    let image = RgbaImage::from_raw(image.width as u32, image.height as u32, image.bytes.clone()).expect("Could not create image from bytes");
    let image = get_rgbasubimage_at(&image, x, y, w, h);
    return Image {
        width: image.width() as u16,
        height: image.height() as u16,
        bytes: image.into_raw(),
    }
}

fn get_rgbasubimage_at(image: &RgbaImage, x: u32, y: u32, w: u32, h: u32) -> RgbaImage {
    return image::SubImage::new(image, x, y, w as u32, h as u32).to_image();
}