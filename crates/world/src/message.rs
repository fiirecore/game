use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq, Eq)]
pub enum MessageColor {
    Black,
    Red,
    Blue,
}

impl Default for MessageColor {
    fn default() -> Self {
        Self::Black
    }
}

impl From<MessageColor> for [f32; 4] {
    fn from(message: MessageColor) -> Self {
        match message {
            MessageColor::Black => [20.0 / 255.0, 20.0 / 255.0, 20.0 / 255.0, 1.0],
            // MessageColor::White => [240.0 / 255.0, 240.0 / 255.0, 240.0 / 255.0, 1.0],
            MessageColor::Red => [0.90, 0.16, 0.22, 1.0],
            MessageColor::Blue => [48.0 / 255.0, 80.0 / 255.0, 200.0 / 255.0, 1.0],
        }
    }
}

type Inner = tinystr::TinyAsciiStr<4>;

#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct MessageTheme(Option<Inner>);

impl MessageTheme {
    pub fn new<S: AsRef<str>>(value: Option<S>) -> Result<Self, tinystr::TinyStrError> {
        Ok(match value {
            Some(i) => Self(Some(Inner::from_str(i.as_ref())?)),
            None => Self::default(),
        })
    }

    pub fn into_inner(self) -> Option<Inner> {
        self.0
    }
}

impl From<Inner> for MessageTheme {
    fn from(inner: Inner) -> Self {
        Self(Some(inner))
    }
}

impl From<Option<Inner>> for MessageTheme {
    fn from(inner: Option<Inner>) -> Self {
        Self(inner)
    }
}
