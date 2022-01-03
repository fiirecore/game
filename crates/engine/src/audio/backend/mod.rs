use fiirengine::error::FileError;

use crate::utils::HashMap;

use crate::audio::{MusicId, SoundId, SoundVariant};

pub mod music;
pub mod sound;

pub type Audio = fiirengine::audio::Sound;
pub type Handle = fiirengine::audio::SoundHandle;

type GameAudioMap<K, V = Audio> = HashMap<K, V>;

#[derive(Default)]
pub struct AudioContext {
    pub(crate) music: GameAudioMap<MusicId>,
    pub(crate) current_music: Option<(MusicId, Handle)>,
    pub(crate) sounds: GameAudioMap<(SoundId, SoundVariant)>,
}

fn add<K: Eq + std::hash::Hash>(
    map: &mut GameAudioMap<K>,
    k: K,
    data: &[u8],
) -> Result<(), FileError> {
    let audio = Audio::new(data)?;
    map.insert(k, audio);
    Ok(())
}

// pub struct Audio(pub(crate) macroquad::audio::Sound);

// impl Audio {
//     pub async fn crate_new(data: &[u8]) -> Result<Self, macroquad::file::FileError> {
//         macroquad::audio::load_sound_from_bytes(data)
//             .await
//             .map(Self)
//     }

//     pub async fn new() {}
// }
