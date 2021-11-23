#[derive(Debug)]
pub enum DataError {
    Serialization(ron::Error),
    IOError(std::io::Error),
    Macroquad(macroquad::prelude::FileError),
}

impl std::error::Error for DataError {}

impl core::fmt::Display for DataError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Debug::fmt(self, f)        
    }
}

impl From<std::io::Error> for DataError {
    fn from(error: std::io::Error) -> Self {
        Self::IOError(error)
    }
}

impl From<ron::Error> for DataError {
    fn from(error: ron::Error) -> Self {
        Self::Serialization(error)
    }
}

impl From<macroquad::prelude::FileError> for DataError {
    fn from(error: macroquad::prelude::FileError) -> Self {
        Self::Macroquad(error)
    }
}