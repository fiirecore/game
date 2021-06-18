use crate::audio::{
    serialized::{SerializedMusicData, SerializedSoundData},
    error::AddAudioError,
};

pub fn create() -> Result<(), AddAudioError> {
    Ok(())
}

pub fn add_music(_: SerializedMusicData) -> Result<(), AddAudioError> {
    Ok(())
}

pub fn add_sound(_: SerializedSoundData) -> Result<(), AddAudioError> {
    Ok(())
}