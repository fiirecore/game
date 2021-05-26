use crate::audio::error::AddAudioError;

pub fn create() -> Result<(), AddAudioError> {
    Ok(())
}

pub fn add_track(_: firecore_audio_lib::serialized::SerializedMusicData) -> Result<(), AddAudioError> {
    Ok(())
}

pub fn add_sound(_: firecore_audio_lib::serialized::SerializedSoundData) -> Result<(), AddAudioError> {
    Ok(())
}