#[cfg(feature = "audio")]
pub(crate) mod backend;

pub mod error;

pub use firecore_audio::*;

use notan::{app::App, prelude::Plugins};

#[cfg(feature = "audio")]
use notan::log::{error, warn};

#[cfg_attr(not(feature = "audio"), allow(unused_variables))]
pub fn play_music(app: &mut App, plugins: &mut Plugins, id: &MusicId) {
    #[cfg(feature = "audio")]
    if let Err(err) = backend::music::play_music(app, plugins, id) {
        warn!("Could not play music id {} with error {}", id, err);
    }
}

#[cfg_attr(not(feature = "audio"), allow(unused_variables))]
pub fn get_current_music(plugins: &Plugins) -> Option<MusicId> {
    #[cfg(feature = "audio")]
    {
        plugins
            .get::<backend::AudioContext>()
            .map(|actx| actx.current_music.as_ref().map(|(id, _)| *id))
            .flatten()
    }
    #[cfg(not(feature = "audio"))]
    {
        None
    }
}

#[cfg_attr(not(feature = "audio"), allow(unused_variables))]
pub fn stop_music(app: &mut App, plugins: &mut Plugins) {
    #[cfg(feature = "audio")]
    backend::music::stop_music(app, plugins);
}

#[cfg_attr(not(feature = "audio"), allow(unused_variables))]
pub fn play_sound(app: &mut App, plugins: &Plugins, sound: SoundId, variant: SoundVariant) {
    #[cfg(feature = "audio")]
    if let Err(err) = backend::sound::play_sound(app, plugins, sound, variant) {
        warn!(
            "Could not play sound {}, variant {:?} with error {}",
            sound, variant, err
        );
    }
}

#[allow(unused_variables)]
pub fn add_music(app: &mut App, plugins: &mut Plugins, id: MusicId, data: Vec<u8>) {
    #[cfg(feature = "audio")]
    if let Err(err) = backend::music::add_music(app, plugins, id, data) {
        error!("Cannot add audio with error {}", err)
    }
}

#[allow(unused_variables)]
pub fn add_sound(
    app: &mut App,
    plugins: &mut Plugins,
    id: SoundId,
    variant: SoundVariant,
    data: Vec<u8>,
) {
    #[cfg(feature = "audio")]
    if let Err(err) = backend::sound::add_sound(app, plugins, id, variant, data) {
        error!("Cannot add sound with error {}", err);
    }
}
