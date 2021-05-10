use serde::{Deserialize, Serialize};

pub mod message;

pub type FontId = u8;

#[derive(Debug, Deserialize, Serialize)]
pub struct FontSheetData {

    pub id: FontId,
    pub width: u8,
    pub height: u8,
    pub chars: String,
    pub custom: Vec<CustomChar>,

}

#[derive(Debug, Deserialize, Serialize)]
pub struct CustomChar {

    pub id: char,
    pub width: u8,
    pub height: Option<u8>,

}

#[derive(Deserialize, Serialize)]
pub struct FontSheet {

    pub image: Vec<u8>,
    pub data: FontSheetData,

}

#[derive(Deserialize, Serialize)]
pub struct FontSheetFile {

    pub file: String,
    pub data: FontSheetData,

}

#[derive(Deserialize, Serialize)]
pub struct SerializedFonts {

    pub fonts: Vec<FontSheet>,

}

pub const fn default_font_id() -> FontId {
    1
}