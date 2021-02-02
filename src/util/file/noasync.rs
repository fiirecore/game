use macroquad::prelude::warn;
use crate::util::texture::Texture;

lazy_static::lazy_static! {
	static ref FILE: parking_lot::Mutex<Option<Vec<u8>>> = parking_lot::Mutex::new(None); // lol
}

pub fn read_noasync<P: AsRef<std::path::Path>>(path: P) -> Option<Vec<u8>> {
    let path = path.as_ref().to_str().unwrap().to_owned();
    
    macroquad::miniquad::fs::load_file(&path.clone(), move |bytes| {
        match bytes {
            Ok(bytes) => *FILE.lock() = Some(bytes),
            Err(err) => {
                warn!("Could not read file at {:?} with error {}", path, err);
            }
        }
    });
    return FILE.lock().take();
}

pub fn read_to_string_noasync<P: AsRef<std::path::Path>>(path: P) -> Option<String> {
    let path = path.as_ref();
    match read_noasync(path) {
        Some(bytes) => {
            match std::str::from_utf8(bytes.as_slice()) {
                Ok(str) => return Some(str.to_string()),
                Err(err) => {
                    warn!("Could not read file at {:?} to string with error {}", path, err);
                    return None;
                }
            }
        }
        None => {
            return None;
        }
    }
}

pub fn open_image_noasync<P: AsRef<std::path::Path>>(path: P) -> Option<macroquad::prelude::Image> {
    let path = path.as_ref();
    match read_noasync(path) {
        Some(bytes) => Some(crate::util::image::byte_image(bytes.as_slice())),
        None => {
            macroquad::prelude::warn!("Could not read image bytes at {:?} with error", path);
            return None;
        }
    }
}

pub fn load_texture_noasync<P: AsRef<std::path::Path>>(path: P) -> Texture {
	let path = path.as_ref();
	return match read_noasync(path) {
	    Some(bytes) => crate::util::texture::byte_texture(bytes.as_slice()),
	    None => {
			macroquad::prelude::warn!("Could not read texture at path {:?} with error", path);
			crate::util::texture::debug_texture()
		}
	}	
}