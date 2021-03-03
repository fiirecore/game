pub mod music;
pub mod sound;
#[cfg(not(target_arch = "wasm32"))]
pub mod kira;
#[cfg(target_arch = "wasm32")]
pub mod quadsnd;

pub fn create() {
    #[cfg(not(target_arch = "wasm32"))]
    macroquad::prelude::collections::storage::store(kira::context::AudioContext::new());
    #[cfg(target_arch = "wasm32")]
    quadsnd::bind_gamefreak().await;
}

pub async fn bind_world_music() {
    #[cfg(not(target_arch = "wasm32"))]
    self::kira::bind_world_music();
    #[cfg(target_arch = "wasm32")]
    self::quadsnd::bind_world_music().await;
}

pub fn play_music(music: music::Music) {
    #[cfg(not(target_arch = "wasm32"))]
    self::kira::context::music::MUSIC_CONTEXT.play_music(music);
    #[cfg(target_arch = "wasm32")]
    self::quadsnd::context::music::play_music(music);
}

pub fn get_music_playing() -> Result<music::Music, AudioError> {
    #[cfg(not(target_arch = "wasm32"))]
    return self::kira::context::music::MUSIC_CONTEXT.get_music_playing();
    #[cfg(target_arch = "wasm32")]
    return self::quadsnd::context::music::get_current_music();
}

#[derive(Debug)]
pub enum AudioError {
    Missing,
    Inaccessable(String)
}

impl std::error::Error for AudioError {}

impl std::fmt::Display for AudioError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            AudioError::Missing => {
                "No sound was found".to_string()
            }
            AudioError::Inaccessable(string) => {
                string.clone()
            }
        })
    }
}

// pub fn play_sound(sound: sound::Sound) {

// }

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