use kira::instance::handle::InstanceHandle;
use kira::sound::SoundSettings;
use kira::sound::handle::SoundHandle;
use macroquad::prelude::warn;
use dashmap::DashMap as HashMap;
use parking_lot::RwLock;
use crate::audio::music::Music;

lazy_static::lazy_static! {
    pub static ref MUSIC_CONTEXT: MusicContext = MusicContext::default();
}

#[derive(Default)]
pub struct MusicContext {

    pub music_map: HashMap<Music, SoundHandle>,
    current_music: RwLock<Option<(Music, InstanceHandle)>>,

}

impl MusicContext {
    
    pub fn play_music(&self, music: Music) {
        if let Some(mut current_music) = self.current_music.try_write() {
            if let Some(instance) = current_music.take() {
                super::stop_instance(instance.0, instance.1);
            }
            match self.music_map.get_mut(&music) {
                Some(mut sound) => {
                    match sound.play(kira::instance::InstanceSettings {
                        loop_start: kira::instance::InstanceLoopStart::Custom(music.loop_start().unwrap_or_default()),
                        ..Default::default()
                    }) {
                        Ok(instance) => {
                            macroquad::prelude::debug!("Playing music: \"{:?}\"", music);
                            *current_music = Some((music, instance));
                        }
                        Err(err) => warn!("Problem playing music \"{:?}\" with error {}", music, err),
                    }
                }
                None => warn!("Could not get music for \"{:?}\"", music),
            }
        } else {
            warn!("Could not unlock write for current music RwLock!");
        }       
    }

    pub fn get_music_playing(&self) -> Option<Music> {
        if let Some(current_music) = self.current_music.try_read() {
            if let Some(ref instance) = *current_music {
                return Some(instance.0);
            }
        } else {
            warn!("Could not unlock read for current music RwLock!");
        }
        return None;
    }
    
}

impl Music {

    pub fn settings(&self) -> SoundSettings {
        SoundSettings {
            default_loop_start: self.loop_start(),
            ..Default::default()
        }
    }

}