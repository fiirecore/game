use std::path::Path;
use macroquad::miniquad::fs::Error;
use macroquad::prelude::warn;
use crate::util::graphics::Texture;

lazy_static::lazy_static! {
	static ref FILE: parking_lot::Mutex<Option<Result<Vec<u8>, Error>>> = parking_lot::Mutex::new(None); // lol
}

pub fn read_noasync<P: AsRef<Path>>(path: P) -> Option<Vec<u8>> {
    let path2 = path.as_ref().clone();
    match read_noasync_result(&path) {
        Ok(bytes) => Some(bytes),
        Err(err) => {
            warn!("Could not read file at {:?} with error {}", path2, err);
            None
        }
    }
}

fn read_noasync_result<P: AsRef<Path>>(path: P) -> Result<Vec<u8>, Error> {
    macroquad::miniquad::fs::load_file(&path.as_ref().to_string_lossy(), move |bytes| {
        *FILE.lock() = Some(bytes);
    });
    match FILE.lock().take() {
        Some(result) => return result,
        None => {
            warn!("Could not take file result from mutex!");
            return Err(Error::DownloadFailed);
        }
    }
    
}

pub fn read_to_string_noasync<P: AsRef<Path>>(path: P) -> Option<String> {
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
	    Some(bytes) => crate::util::graphics::texture::byte_texture(bytes.as_slice()),
	    None => {
			macroquad::prelude::warn!("Could not read texture at path {:?} with error", path);
			crate::util::graphics::texture::debug_texture()
		}
	}	
}