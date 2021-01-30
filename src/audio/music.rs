use ahash::AHashMap;
use kira::sound::handle::SoundHandle;
use macroquad::prelude::info;
use macroquad::prelude::warn;
use enum_iterator::IntoEnumIterator;
use kira::sound::SoundSettings;
use super::Music;

impl Music {

    pub fn bind_music() {
        let mut map = AHashMap::new();
        let mut manager = crate::audio::AUDIO_MANAGER.lock();    
        info!("Loading music...");
        for music in Music::into_enum_iter() {
            if music != Music::IntroGamefreak {
                match manager.load_sound(String::from("music/") + &music.to_string() + ".ogg", SoundSettings::default()) {
                    Ok(sound) => {
                        map.insert(music, sound);
                        info!("Loaded {} successfully", music);
                    }
                    Err(err) => {
                        warn!("Problem loading music {} with error {}", music, err);
                    }
                }
            }
        }
        info!("Finished loading world music!");
        *super::MUSIC_MAP.lock() = map;
    }

    pub fn bind_gf() -> Option<SoundHandle> {
        let mut manager = crate::audio::AUDIO_MANAGER.lock();    
        match manager.load_sound("music/gamefreak.ogg", SoundSettings::default()) {
            Ok(sound) => return Some(sound),
            Err(err) => {
                warn!("Could not load gamefreak intro music with error {}", err);
                return None;
            }
        }
    }

}