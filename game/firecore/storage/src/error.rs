#[derive(Debug)]
pub enum DataError {
    
    NoFileName,
    ReadOnly,

    NoDirectory,
    Serialize(ron::Error),
    Deserialize(&'static str, ron::Error),

    IOError(std::io::Error),
    FileError(macroquad::file::FileError),

}

impl std::error::Error for DataError {}

impl core::fmt::Display for DataError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            this => std::fmt::Debug::fmt(this, f),
        }
        
    }
}

impl From<std::io::Error> for DataError {
    fn from(error: std::io::Error) -> Self {
        Self::IOError(error)
    }
}

impl From<ron::Error> for DataError {
    fn from(error: ron::Error) -> Self {
        Self::Serialize(error)
    }
}

impl From<macroquad::file::FileError> for DataError {
    fn from(error: macroquad::file::FileError) -> Self {
        Self::FileError(error)
    }
}