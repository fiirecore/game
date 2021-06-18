use deps::tetra::Context;
use super::audio::serialized::SerializedSoundData;
use super::error::{AddAudioError, PlayAudioError};

pub use super::audio::sound::*;

pub fn play_sound(ctx: &Context, sound: &Sound) -> Result<(), PlayAudioError> {
    super::backend::sound::play_sound(ctx, &sound)
}

pub fn add_sound(sound_data: SerializedSoundData) -> Result<(), AddAudioError> {
    super::backend::context::add_sound(sound_data)
}