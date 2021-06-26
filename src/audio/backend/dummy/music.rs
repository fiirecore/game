use crate::tetra::Context;
use crate::audio::error::PlayAudioError;

pub fn play_music(_: &Context, music: crate::audio::music::MusicId) -> Result<(), PlayAudioError> {
    Ok(())
}

pub fn get_current_music() -> Option<crate::audio::music::MusicId> {
    None
}