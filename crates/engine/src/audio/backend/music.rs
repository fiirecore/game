use fiirengine::error::FileError;

use fiirengine::audio::{self, PlaySoundParams};

use crate::{
    audio::{error::PlayAudioError, MusicId},
    Context, EngineContext,
};

use super::{add, GameAudioMap};

pub fn add_music(
    music: &mut GameAudioMap<MusicId>,
    id: MusicId,
    data: Vec<u8>,
) -> Result<(), FileError> {
    add(music, id, &data)
}

pub fn play_music(ctx: &mut Context, eng: &mut EngineContext, music: &MusicId) -> Result<(), PlayAudioError> {
    stop_music(ctx, eng);
    match eng.audio.music.get(music) {
        Some(audio) => {
            let handle = audio::play_sound(
                ctx,
                audio,
                PlaySoundParams {
                    looped: true,
                    volume: 0.5,
                },
            );
            eng.audio.current_music = Some((*music, handle));
            Ok(())
            // match audio.play(ctx) {
            // Ok(instance) => {
            //     instance.set_repeating(true);
            //     instance.set_volume(0.3);
            //     ctx.audio.current_music = Some((music, instance));
            //     Ok(())
            // }
            // Err(err) => Err(PlayAudioError::TetraError(err)),
        }
        None => Err(PlayAudioError::Missing),
    }
}

pub fn stop_music(ctx: &mut Context, eng: &mut EngineContext) {
    if let Some((_, instance)) = eng.audio.current_music.take() {
        audio::stop_sound(ctx, instance);
    }
}
