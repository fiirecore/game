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
    current_music: Option<InstanceHandle>,

}

impl MusicContext {
    
    pub fn play_music(&mut self, music: Music) {
        if let Some(instance) = self.current_music.take() {
            super::stop_instance(music, instance);
        }
        match self.music_map.get_mut(&music) {
            Some(sound) => {
                match sound.play(kira::instance::InstanceSettings {
                    loop_start: kira::instance::InstanceLoopStart::Custom(music.loop_start().unwrap_or_default()),
                    ..Default::default()
                }) {
                    Ok(instance) => {
                        self.current_music = Some(instance);
                    }
                    Err(err) => warn!("Problem playing music {} with error {}", music, err),
                }
            }
            None => warn!("Could not get sound for {}", music),
        }        
    }

    pub fn is_music_playing(&self) -> bool {
        return self.current_music.is_some();
    }

}