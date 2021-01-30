use std::fmt::Display;

use ahash::AHashMap;
use kira::instance::handle::InstanceHandle;
use kira::manager::AudioManager;
use macroquad::prelude::warn;
use parking_lot::Mutex;
use enum_iterator::IntoEnumIterator;

use self::sound::Sound;

mod music;
mod sound;

lazy_static::lazy_static! {
    pub static ref AUDIO_MANAGER: Mutex<AudioManager> = Mutex::new(AudioManager::new(kira::manager::AudioManagerSettings::default()).expect("Could not create audio manager"));
    static ref MUSIC_MAP: Mutex<AHashMap<Music, kira::sound::handle::SoundHandle>> = Mutex::new(AHashMap::new());
    static ref MUSIC_INSTANCE: Mutex<Option<InstanceHandle>> = Mutex::new(None);
    static ref SOUND_INSTANCE_MAP: Mutex<AHashMap<Sound, InstanceHandle>> = Mutex::new(AHashMap::new());
}

pub fn play_music(music: Music) {
    if let Some(instance) = MUSIC_INSTANCE.lock().take() {
        stop_instance(music, instance);
    }
    match MUSIC_MAP.lock().get_mut(&music) {
        Some(sound) => {
            match sound.play(kira::instance::InstanceSettings::default()) {
                Ok(instance) => {
                    *MUSIC_INSTANCE.lock() = Some(instance);
                }
                Err(err) => warn!("Problem playing music {} with error {}", music, err),
            }
        }
        None => warn!("Could not get sound for {}", music),
    }
}

pub fn is_music_playing() -> bool {
    MUSIC_INSTANCE.lock().is_some()
}

pub fn stop_sound(sound: Sound) {
    let mut instances = SOUND_INSTANCE_MAP.lock();
    match instances.remove(&sound) {
        Some(instance) => {
            stop_instance(sound, instance);
        },
        None => warn!("Could not get sound instance handle for {}, probably not playing", sound),
    }
}

pub fn stop_all_sounds() {
    let sound_keys: Vec<Sound> = SOUND_INSTANCE_MAP.lock().keys().into_iter().map(|music|*music).collect();
    for sound in sound_keys {
        stop_sound(sound);
    }
}

fn stop_instance(audio: impl Display, mut instance: InstanceHandle) {
    if let Err(err) = instance.stop(kira::instance::StopInstanceSettings::default()) {
        warn!("Problem stopping audio instance {} with error {}", audio, err);
    }
}

#[derive(Copy, Clone, Hash, PartialEq, Eq, IntoEnumIterator)]
pub enum Music {

    IntroGamefreak,
    // IntroPokemon,
    Title,

    Pallet,
    Pewter,
    Fuchsia,
    Lavender,
    Celadon,
    Cinnabar,
    Vermilion,

    Route1,
    Route2,
    Route3,
    Route4,

    ViridianForest,
    MountMoon,
    Gym,

    BattleWild,
    BattleTrainer,
    BattleGym,
    // BattleChampion,

}

impl Default for Music {
    fn default() -> Self {
        Music::Pallet
    }
}

impl Music {

    pub fn loop_start(&self) -> Option<f64> {
        match *self {
            Music::BattleWild => Some(13.75),
            _ => None,
        }
    }

    pub fn len(&self) -> Option<f64> {
        match *self {
            Music::Route1 => Some(25.0),
            _ => None,
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