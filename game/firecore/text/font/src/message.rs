use serde::{Deserialize, Serialize};

pub type MessagePages = Vec<MessagePage>;
pub type Lines = Vec<String>; // maybe use cow<'_ str>

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Message {

    pub pages: MessagePages,

    #[serde(default)]
    pub color: TextColor,

}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct MessagePage {

    pub lines: Lines,  

    pub wait: Option<f32>,

}

#[derive(Debug, Copy, Clone, Hash, Deserialize, Serialize)]
pub enum TextColor {

    White,
    Gray,
    Black,
    Red,
    Blue,

}

impl Default for TextColor {
    fn default() -> Self {
        Self::Black
    }
}

impl Message {

    pub fn empty(color: TextColor, len: usize) -> Self {
        Self::new(color, MessagePages::with_capacity(len))
    }

    pub fn new(color: TextColor, pages: MessagePages) -> Self {
        Self {
            pages,
            color,
        }
    }

    pub fn single(lines: Lines, color: TextColor, wait: Option<f32>) -> Self {
        Self::new(color, vec![MessagePage::new(lines, wait)])
    }

}

impl MessagePage {
    
    pub fn new(lines: Lines, wait: Option<f32>) -> Self {
        Self {
            lines,
            wait,
        }
    }

}