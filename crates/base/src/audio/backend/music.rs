use notan::prelude::{App, Plugins};

use crate::audio::{error::PlayAudioError, MusicId};

use super::add;

pub fn add_music(
    app: &mut App,
    plugins: &mut Plugins,
    // music: &mut GameAudioMap<MusicId>,
    id: MusicId,
    data: Vec<u8>,
) -> Result<(), String> {
    plugins
        .get_mut::<super::AudioContext>()
        .map(|mut actx| add(app, &mut actx.music, id, &data))
        .ok_or_else(|| String::from("Could not get audio context to create music"))?
}

pub fn play_music(
    ctx: &mut App,
    plugins: &mut Plugins,
    music: &MusicId,
) -> Result<(), PlayAudioError> {
    stop_music(ctx, plugins);
    match plugins.get_mut::<super::AudioContext>() {
        Some(mut actx) => match actx.music.get(music) {
            Some(audio) => {
                let handle = ctx.audio.play_sound(audio, actx.volume, true);
                actx.current_music = Some((*music, handle));
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
        },
        None => todo!(),
    }
}

pub fn stop_music(ctx: &mut App, plugins: &mut Plugins) {
    if let Some((_, instance)) = plugins
        .get_mut::<super::AudioContext>()
        .map(|mut audio| audio.current_music.take())
        .flatten()
    {
        ctx.audio.stop(&instance);
    }
}
