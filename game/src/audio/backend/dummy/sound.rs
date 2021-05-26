use crate::tetra::Context;
use crate::audio::error::PlayAudioError;

pub fn play_sound(_: &Context, _: &crate::audio::Sound) -> Result<(), PlayAudioError> {
    Ok(())
}