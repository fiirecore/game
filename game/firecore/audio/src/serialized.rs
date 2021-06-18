use serde::{Deserialize, Serialize};

use crate::music::Music;
use crate::sound::Sound;

#[derive(Serialize, Deserialize)]
pub struct SerializedAudio {

    pub music: Vec<SerializedMusicData>,
    pub sounds: Vec<SerializedSoundData>,

}

#[cfg(feature = "file")]
#[derive(Serialize, Deserialize)]
pub struct SerializedMusicFile {

    pub file: String,
    pub music: Music,

}

#[derive(Serialize, Deserialize)]
pub struct SerializedMusicData {

    pub bytes: Vec<u8>,
    pub music: Music,

}

#[cfg(feature = "file")]
#[derive(Serialize, Deserialize)]
pub struct SerializedSoundFile {

    pub file: String,
    pub sound: Sound,

}

#[derive(Serialize, Deserialize)]
pub struct SerializedSoundData {

    pub bytes: Vec<u8>,
    pub sound: Sound,

}