use kira::instance::handle::InstanceHandle;
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