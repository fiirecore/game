// use fiirengine::error::FileError;
use notan::prelude::{App, Plugin};

use crate::utils::HashMap;

use crate::audio::{MusicId, SoundId, SoundVariant};

pub mod music;
pub mod sound;

pub type Audio = notan::audio::AudioSource;
pub type Handle = notan::audio::Sound;

type GameAudioMap<K, V = Audio> = HashMap<K, V>;

#[derive(Default)]
pub struct AudioContext {
    pub(crate) music: GameAudioMap<MusicId>,
    pub(crate) current_music: Option<(MusicId, Handle)>,
    pub(crate) sounds: GameAudioMap<(SoundId, SoundVariant)>,
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
