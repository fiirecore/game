use crate::audio::{backend::add, error::PlayAudioError, SoundId, SoundVariant};

use notan::prelude::{App, Plugins};

pub fn add_sound(
    ctx: &mut App,
    plugins: &mut Plugins,
    // sounds: &mut GameAudioMap<(SoundId, SoundVariant)>,
    id: SoundId,
    variant: SoundVariant,
    data: Vec<u8>,
) -> Result<(), String> {
    plugins
        .get_mut::<super::AudioContext>()
        .map(|mut actx| add(ctx, &mut actx.sounds, (id, variant), &data))
        .ok_or_else(|| String::from("Could not get audio context to create sound"))?
}

pub fn play_sound(
    ctx: &mut App,
    plugins: &Plugins,
    sound: SoundId,
    variant: SoundVariant,
) -> Result<(), PlayAudioError> {
    match plugins.get::<super::AudioContext>() {
        Some(actx) => match actx.sounds.get(&(sound, variant)) {
            Some(handle) => {
                ctx.audio.play_sound(handle, 0.5, false);
                Ok(())
                // match  handle.play(ctx) {
                //     Ok(instance) => {
                //         instance.set_volume(0.3);
                //         Ok(())
                //     }
                //     Err(err) => {
                //         Err(PlayAudioError::TetraError(err))
                //     }
                // }
            }
            None => Err(PlayAudioError::Missing),
        },
        None => Err(PlayAudioError::Uninitialized),
    }
}
