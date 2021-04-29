use serde::{Deserialize, Serialize};

use crate::FontId;

pub type MessageSet = Vec<MessagePage>;
pub type Lines = Vec<String>;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Message {

    #[serde(default = "default_font_id")]
    pub font: FontId,

    pub message_set: MessageSet,

    #[serde(default)]
    pub color: TextColor,

}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct MessagePage {

    pub lines: Lines,  

    #[serde(default)]
    pub wait: Option<f32>,

}

const fn default_font_id() -> FontId {
    1
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

    pub fn empty(font: FontId, color: TextColor) -> Self {
        Self::new(font, color, MessageSet::default())
    }

    pub fn new(font: FontId, color: TextColor, message_set: MessageSet) -> Self {
        Self {
            font,
            message_set,
            color,
        }
    }

    pub fn single(lines: Lines, color: TextColor, wait: Option<f32>) -> Self {
        Self::new(default_font_id(), color, vec![MessagePage::new(lines, wait)])
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