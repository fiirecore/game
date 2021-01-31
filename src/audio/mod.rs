use std::fmt::Display;
use std::ops::DerefMut;
use kira::sound::SoundSettings;
use macroquad::prelude::collections::storage::get_mut;
use enum_iterator::IntoEnumIterator;

use crate::util::audio::AudioContext;

pub mod music;
mod sound;

pub fn play_music(music: Music) {
    if let Some(mut ac) = get_mut::<AudioContext>() {
        ac.deref_mut().play_music(music);
    }
}

pub fn is_music_playing() -> bool {
    if let Some(mut ac) = get_mut::<AudioContext>() {
        return ac.deref_mut().is_music_playing();
    } else {
        return false;
    }
}

pub fn bind_world_music() {
    if cfg!(not(target_arch = "wasm32")) {
        std::thread::spawn( || {
            if let Some(mut ac) = get_mut::<AudioContext>() {
                ac.deref_mut().bind_music();     
            }
        });
    } else {
        if let Some(mut ac) = get_mut::<AudioContext>() {
            ac.deref_mut().bind_music();     
        }
    }
    
}

// pub fn stop_sound(sound: Sound) {
//     let mut instances = SOUND_INSTANCE_MAP.lock();
//     match instances.remove(&sound) {
//         Some(instance) => {
//             stop_instance(sound, instance);
//         },
//         None => warn!("Could not get sound instance handle for {}, probably not playing", sound),
//     }
// }


// pub fn stop_all_sounds() {
//     let sound_keys: Vec<Sound> = SOUND_INSTANCE_MAP.lock().keys().into_iter().map(|music|*music).collect();
//     for sound in sound_keys {
//         stop_sound(sound);
//     }
// }

#[derive(Copy, Clone, Hash, PartialEq, Eq, IntoEnumIterator)]
pub enum Music {

    IntroGamefreak,
    // IntroPokemon,
    Title, // 45.010

    Pallet, // 43.640
    Pewter,
    Fuchsia,
    Lavender,
    Celadon,
    Cinnabar,
    Vermilion,

    Route1, // 25.090
    Route2,
    Route3,
    Route4,

    ViridianForest,
    MountMoon,
    Gym,

    BattleWild, // 44.480
    BattleTrainer, // 1:41.870
    BattleGym, // 56.780
    // BattleChampion,

}

impl Default for Music {
    fn default() -> Self {
        Music::Pallet
    }
}

impl Music {

    pub fn included_bytes(&self) -> Option<&[u8]> {
        match *self {
            Music::IntroGamefreak => Some(include_bytes!("../../assets/music/gamefreak.ogg")),
            Music::Title => Some(include_bytes!("../../assets/music/title.ogg")),
            Music::Pallet => Some(include_bytes!("../../assets/music/pallet.ogg")),
            Music::BattleWild => Some(include_bytes!("../../assets/music/vs_wild.ogg")),
            _ => None,
        }
    }

    pub fn loop_start(&self) -> Option<f64> {
        match *self {
            Music::BattleWild => Some(13.15),
            _ => None,
        }
    }

    pub fn settings(&self) -> SoundSettings {
        SoundSettings {
            default_loop_start: self.loop_start(),
            ..Default::default()
        }
    }

}

impl From<u8> for Music {
    fn from(id: u8) -> Self {
        match id {
            0x1F => Music::ViridianForest,
            0x13 => Music::Gym,
            0x20 => Music::MountMoon,
            0x23 => Music::Route1,
            0x24 => Music::Route2,
            0x25 => Music::Route3,
            0x26 => Music::Route4,
            0x34 => Music::Fuchsia,
            0x3A => Music::Pewter,
            0x18 => Music::Lavender,
            0x35 => Music::Celadon,
            0x17 => Music::Cinnabar,
            0x39 => Music::Vermilion,
            0x2C => Music::Pallet,
            _ => {
                macroquad::prelude::warn!("Could not get music with id #{}!", id);
                return Music::default();
            },
        }
    }
}

impl Display for Music {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match *self {

            Music::IntroGamefreak => "gamefreak",
            Music::Title => "title",

            Music::Pallet => "pallet",
            Music::Pewter => "pewter",
            Music::Fuchsia => "fuchsia",
            Music::Lavender => "lavender",
            Music::Celadon => "celadon",
            Music::Cinnabar => "cinnabar",
            Music::Vermilion => "vermilion",

            Music::Route1 => "route1",
            Music::Route2 => "route2",
            Music::Route3 => "route3",
            Music::Route4 => "route4",

            Music::Gym => "gym",
            Music::ViridianForest => "viridian_forest",
            Music::MountMoon => "mt_moon",

            Music::BattleWild => "vs_wild",
            Music::BattleTrainer => "vs_trainer",
            Music::BattleGym => "vs_gym",

        })
    }
}

pub fn from_ogg_bytes(bytes: &[u8], settings: kira::sound::SoundSettings) -> Result<kira::sound::Sound, kira::sound::error::SoundFromFileError> {
    use lewton::samples::Samples;
    let mut reader = lewton::inside_ogg::OggStreamReader::new(std::io::Cursor::new(bytes))?;
    let mut stereo_samples = vec![];
    while let Some(packet) = reader.read_dec_packet_generic::<Vec<Vec<f32>>>()? {
        let num_channels = packet.len();
        let num_samples = packet.num_samples();
        match num_channels {
            1 => {
                for i in 0..num_samples {
                    stereo_samples.push(kira::Frame::from_mono(packet[0][i]));
                }
            }
            2 => {
                for i in 0..num_samples {
                    stereo_samples.push(kira::Frame::new(packet[0][i], packet[1][i]));
                }
            }
            _ => return Err(kira::sound::error::SoundFromFileError::UnsupportedChannelConfiguration),
        }
    }
    Ok(kira::sound::Sound::from_frames(
        reader.ident_hdr.audio_sample_rate,
        stereo_samples,
        settings,
    ))
}