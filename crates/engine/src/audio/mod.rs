#[cfg(feature = "audio")]
pub(crate) mod backend;

pub mod error;

pub type MusicId = tinystr::TinyStr16;
pub type SoundId = tinystr::TinyStr8;
pub type SoundVariant = Option<u16>;

use crate::{Context, EngineContext};

#[cfg_attr(not(feature = "audio"), allow(unused_variables))]
pub fn play_music(ctx: &mut Context, eng: &mut EngineContext, id: &MusicId) {
    #[cfg(feature = "audio")]
    if let Err(err) = backend::music::play_music(ctx, eng, id) {
        crate::log::warn!("Could not play music id {} with error {}", id, err);
    }
}

#[cfg_attr(not(feature = "audio"), allow(unused_variables))]
pub fn get_current_music(eng: &EngineContext) -> Option<&MusicId> {
    #[cfg(feature = "audio")]
    {
        eng.audio.current_music.as_ref().map(|(id, _)| id)
    }
    #[cfg(not(feature = "audio"))]
    {
        None
    }
}

#[cfg_attr(not(feature = "audio"), allow(unused_variables))]
pub fn stop_music(ctx: &mut Context, eng: &mut EngineContext) {
    #[cfg(feature = "audio")]
    backend::music::stop_music(ctx, eng);
}

#[cfg_attr(not(feature = "audio"), allow(unused_variables))]
pub fn play_sound(ctx: &mut Context, eng: &mut EngineContext, sound: &SoundId, variant: Option<u16>) {
    #[cfg(feature = "audio")]
    if let Err(err) = backend::sound::play_sound(ctx, eng, sound, variant) {
        crate::log::warn!(
            "Could not play sound {}, variant {:?} with error {}",
            sound,
            variant,
            err
        );
    }
}

#[allow(unused_variables)]
pub fn add_music(ctx: &mut Context, eng: &mut EngineContext, id: MusicId, data: Vec<u8>) {
    #[cfg(feature = "audio")]
    if let Err(err) = backend::music::add_music(&mut eng.audio.music, id, data) {
        crate::log::error!("Cannot add audio with error {}", err)
    }
}

#[allow(unused_variables)]
pub fn add_sound(ctx: &mut Context, eng: &mut EngineContext, id: SoundId, variant: Option<u16>, data: Vec<u8>) {
    #[cfg(feature = "audio")]
    if let Err(err) = backend::sound::add_sound(&mut eng.audio.sounds, id, variant, data) {
        crate::log::error!("Cannot add sound with error {}", err);
    }
}
