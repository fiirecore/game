use image::RgbaImage;
use macroquad::prelude::Image;
use crate::util::TILE_SIZE;

pub fn byte_image(bytes: &[u8]) -> Image {
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

pub fn get_subimage(image: &Image, id: usize) -> Image {
    return get_subimage_wh(image, id, TILE_SIZE as u32, TILE_SIZE as u32);
}

pub fn get_subimage_wh(image: &Image, id: usize, w: u32, h: u32) -> Image {
	let x = id % (image.width() / w as usize);
	let y = id / (image.width() / w as usize);
	return get_subimage_at(image, x as u32 * w, y as u32 * h, w, h);
}

pub fn get_subimage_at(image: &Image, x: u32, y: u32, w: u32, h: u32) -> Image {
    let image = get_rgbaimage(image);
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

pub fn get_rgbaimage(image: &Image) -> RgbaImage {
    RgbaImage::from_raw(image.width as u32, image.height as u32, image.bytes.clone()).expect("Could not create image from bytes")
}