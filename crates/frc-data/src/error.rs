#[derive(Debug)]
pub enum Error {

    ReadOnly,
    IOError(std::io::Error),
    FileError(macroquad::file::FileError),

}

impl std::error::Error for Error {}

impl core::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self, f)
    }
}