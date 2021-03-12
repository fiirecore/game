// use macroquad::prelude::FileError;

pub mod noasync;

// pub async fn load_file<P: AsRef<std::path::Path>>(path: P) -> Result<Vec<u8>, FileError> {
//     macroquad::prelude::load_file(&*path.as_ref().to_string_lossy()).await
// }

// pub async fn read_to_string<P: AsRef<std::path::Path>>(path: P) -> Result<String, FileError> {
//     return macroquad::prelude::load_string(&*path.as_ref().to_string_lossy()).await;
// }