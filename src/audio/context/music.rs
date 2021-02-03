use kira::instance::handle::InstanceHandle;
use kira::sound::SoundSettings;
use kira::sound::handle::SoundHandle;
use macroquad::prelude::warn;
use parking_lot::Mutex;
use ahash::AHashMap as HashMap;

use crate::audio::music::Music;

lazy_static::lazy_static! {
    pub static ref MUSIC_CONTEXT: Mutex<MusicContext> = Mutex::new(MusicContext::default());
}

#[derive(Default)]
pub struct MusicContext {

    pub music_map: HashMap<Music, SoundHandle>,
    current_music: Option<(Music, InstanceHandle)>,

}

impl MusicContext {
    
    pub fn play_music(&mut self, music: Music) {
        if let Some(instance) = self.current_music.take() {
            super::stop_instance(instance.0, instance.1);
        }
        match self.music_map.get_mut(&music) {
            Some(sound) => {
                match sound.play(kira::instance::InstanceSettings {
                    loop_start: kira::instance::InstanceLoopStart::Custom(music.loop_start().unwrap_or_default()),
                    ..Default::default()
                }) {
                    Ok(instance) => {
                        self.current_music = Some((music, instance));
                    }
                    Err(err) => warn!("Problem playing music {} with error {}", music, err),
                }
            }
            None => warn!("Could not get sound for {}", music),
        }        
    }

    pub fn get_music_playing(&self) -> Option<Music> {
        if let Some(ref instance) = self.current_music {
            Some(instance.0)
        } else {
            None
        }
    }
    

}

impl Music {

    pub fn included_bytes(&self) -> Option<&[u8]> { // To - do: Load dynamically from assets folder instead of specifying this
        match *self {
            Music::IntroGamefreak => Some(include_bytes!("../../../build/assets/music/gamefreak.ogg")),
            Music::Title => Some(include_bytes!("../../../build/assets/music/title.ogg")),
            Music::Pallet => Some(include_bytes!("../../../build/assets/music/pallet.ogg")),
            Music::EncounterBoy => Some(include_bytes!("../../../build/assets/music/encounter_boy.ogg")),
            Music::BattleWild => Some(include_bytes!("../../../build/assets/music/vs_wild.ogg")),
            Music::BattleTrainer => Some(include_bytes!("../../../build/assets/music/vs_trainer.ogg")),
            Music::BattleGym => Some(include_bytes!("../../../build/assets/music/vs_gym.ogg")),
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