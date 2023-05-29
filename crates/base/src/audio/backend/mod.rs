// use fiirengine::error::FileError;
use notan::prelude::{App, Plugin};

use hashbrown::HashMap;

use crate::audio::{MusicId, SoundId, SoundVariant};

pub mod music;
pub mod sound;

pub type Audio = notan::audio::AudioSource;
pub type Handle = notan::audio::Sound;

type GameAudioMap<K, V = Audio> = HashMap<K, V>;

pub struct AudioContext {
    pub(crate) music: GameAudioMap<MusicId>,
    pub(crate) current_music: Option<(MusicId, Handle)>,
    pub(crate) sounds: GameAudioMap<(SoundId, SoundVariant)>,
    pub volume: f32,
}

impl Plugin for AudioContext {}

fn add<K: Eq + std::hash::Hash>(
    ctx: &mut App,
    map: &mut GameAudioMap<K>,
    k: K,
    data: &[u8],
) -> Result<(), String> {
    let audio = ctx.audio.create_source(data)?;
    map.insert(k, audio);
    Ok(())
}

impl Default for AudioContext {
    fn default() -> Self {
        Self { music: Default::default(), current_music: Default::default(), sounds: Default::default(), volume: 0.5 }
    }
}

impl AudioContext {

    pub fn update_volume(&self, app: &mut App) {
        if let Some((.., handle)) = self.current_music.as_ref() {
            app.audio.set_volume(handle, self.volume);
        }
    }

}