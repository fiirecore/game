use fiirengine::{error::FileError, audio::{self, PlaySoundParams}};

use crate::{
    audio::{
        backend::{add, GameAudioMap},
        error::PlayAudioError,
        SoundId, SoundVariant,
    },
    Context, EngineContext,
};

pub fn add_sound(
    sounds: &mut GameAudioMap<(SoundId, SoundVariant)>,
    id: SoundId,
    variant: SoundVariant,
    data: Vec<u8>,
) -> Result<(), FileError> {
    add(sounds, (id, variant), &data)
}

pub fn play_sound(
    ctx: &mut Context,
    eng: &EngineContext,
    sound: &SoundId,
    variant: Option<u16>,
) -> Result<(), PlayAudioError> {
    match eng.audio.sounds.get(&(*sound, variant)) {
        Some(handle) => {
            audio::play_sound(ctx, handle, PlaySoundParams {
                looped: false,
                volume: 1.0,
            });
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
    }
}
