#[derive(Debug)]
pub enum DataError {
    IOError(std::io::Error),
    Utf8Error(std::str::Utf8Error),
    Text(ron::Error),
    Bytes(postcard::Error),
    // #[cfg(target_arch = "wasm32")]
    #[cfg(all(feature = "io", target_arch = "wasm32"))]
    QuadStorageError,
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

impl From<std::str::Utf8Error> for DataError {
    fn from(error: std::str::Utf8Error) -> Self {
        Self::Utf8Error(error)
    }
}

impl From<ron::Error> for DataError {
    fn from(error: ron::Error) -> Self {
        Self::Text(error)
    }
}

impl From<postcard::Error> for DataError {
    fn from(error: postcard::Error) -> Self {
        Self::Bytes(error)
    }
}
