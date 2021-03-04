use serde::{Deserialize, Serialize};

use self::color::TextColor;

pub mod font;
pub mod color;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Message {

    #[serde(default = "default_font")]
    pub font_id: usize,
    pub message: Vec<String>,
    #[serde(default)]
    pub color: TextColor,
    #[serde(default)]
    pub no_pause: bool,

}

impl Default for Message {
    fn default() -> Self {
        Self {
            font_id: 1,
            color: TextColor::default(),
            message: vec![String::from("Default message")],
            no_pause: true,
        }
    }
}

const fn default_font() -> usize {
    1
}

impl Message {

    pub fn new(message: Vec<String>, no_pause: bool,) -> Self {
        Self::with_color(message, no_pause, TextColor::default())
    }

    pub fn with_color(message: Vec<String>, no_pause: bool, color: TextColor) -> Self {
        Self {
            message,
            no_pause,
            color,
            ..Default::default()
        }
    }

}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct MessageSet {
    pub messages: Vec<Message>,
}

impl MessageSet {

    pub fn new(font_id: usize, color: TextColor, messages: Vec<Vec<String>>) -> Self {
        let mut messages_vec = Vec::new();
        for message in messages {
            messages_vec.push(Message {
                font_id: font_id,
                message: message,
                color: color,
                no_pause: false,
            })
        }
        Self {
            messages: messages_vec,
        }
    }

    pub fn get_phrase(&self, index: usize) -> &Message {
        &self.messages[index]
    }

    pub fn len(&self) -> usize {
        self.messages.len()
    }
    
}

impl Default for MessageSet {
    fn default() -> Self {
        Self {
            messages: vec![Message::default()],
        }
    }
}