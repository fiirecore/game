use crate::error::PlayAudioError;

pub fn play_music(music: crate::MusicId) -> Result<(), PlayAudioError> {
    Ok(())
}

pub fn get_current_music() -> Option<crate::MusicId> {
    None
}