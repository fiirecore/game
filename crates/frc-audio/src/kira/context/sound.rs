use dashmap::DashMap as HashMap;
// use kira::instance::handle::InstanceHandle;
use kira::sound::handle::SoundHandle;
use macroquad::prelude::warn;
use crate::sound::Sound;

lazy_static::lazy_static! {
    pub static ref SOUND_CONTEXT: SoundContext = SoundContext::new();
}

pub struct SoundContext {

    sound_map: HashMap<Sound, SoundHandle>,
    // current_sounds: Vec<InstanceHandle>,

}

impl SoundContext {

    pub fn new() -> Self {
        Self {
            sound_map: HashMap::new(),
            // current_sounds: Vec::new(),
        }
    }

    pub fn play_sound(&self, sound: Sound) {
        if let Some(mut sound_handle) = self.sound_map.get_mut(&sound) {
            match sound_handle.play(kira::instance::InstanceSettings::default()) {
                Ok(_handle) => {
                    macroquad::prelude::info!("Played sound {:?}", sound);
                    // self.current_sounds.push(handle);
                }
                Err(err) => {
                    warn!("Could not play sound {:?} with error {}", sound, err);
                }
            }
        } else {
            warn!("Could not get sound for sound id \"{:?}\"", sound);
        }
    }

    pub fn add_sound(&self, sound: Sound, bytes: &[u8]) {
        match crate::kira::from_ogg_bytes(bytes, kira::sound::SoundSettings::default()) {
            Ok(sound_handle) => {
                match super::AUDIO_CONTEXT.audio_manager.lock().as_mut() {
                    Some(context) => {
                        match context.add_sound(sound_handle) {
                            Ok(sound_handle) => {
                                self.sound_map.insert(
                                    sound,
                                    sound_handle,
                                );
                                macroquad::prelude::info!("Loaded sound {:?}", sound);
                            }
                            Err(err) => {
                                warn!("Could not add sound to audio manager with error {}", err);
                            }
                        }
                    }
                    None => {
                        warn!("Could not add sound, audio manager doesn't exist!");
                    }
                }
            }
            Err(err) => {
                warn!("Could not decode sound {:?} from bytes with error {}", sound, err);
            }
        }
        
    }

}