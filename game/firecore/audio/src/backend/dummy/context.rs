use crate::error::AddAudioError;

pub fn create() -> Result<(), AddAudioError> {
    Ok(())
}

pub fn add_track(_: crate::SerializedMusicData) -> Result<(), AddAudioError> {
    Ok(())
}

pub fn add_sound(_: crate::SerializedSoundData) -> Result<(), AddAudioError> {
    Ok(())
}