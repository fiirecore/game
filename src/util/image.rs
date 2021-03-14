use macroquad::prelude::Image;

pub fn byte_image(bytes: &[u8]) -> Result<Image, image::ImageError> {
    from_file_with_format(bytes, Some(image::ImageFormat::Png))
}

pub fn from_file_with_format(bytes: &[u8], format: Option<image::ImageFormat>) -> Result<Image, image::ImageError> {
    match format {
        Some(fmt) =>
            image::load_from_memory_with_format(&bytes, fmt)
            .map(|img| from_rgba8(img.to_rgba8()) ),
        None => image::load_from_memory(&bytes).map(|img| from_rgba8(img.to_rgba8()) ),
    }        
}

fn from_rgba8(img: image::RgbaImage) -> Image {
    Image {
        width: img.width() as u16,
        height: img.height() as u16,
        bytes: img.into_raw(),
    }
}