use self::music::Music;

pub mod music;
pub mod sound;
#[cfg(not(target_arch = "wasm32"))]
pub mod kira;
#[cfg(target_arch = "wasm32")]
pub mod quadsnd;

pub async fn bind_world_music() {
    #[cfg(not(target_arch = "wasm32"))]
    self::kira::bind_world_music();
    #[cfg(target_arch = "wasm32")]
    self::quadsnd::bind_world_music().await;
}

pub fn play_music(music: Music) {
    macroquad::prelude::debug!("Playing {:?}", music);
    #[cfg(not(target_arch = "wasm32"))]
    self::kira::context::music::MUSIC_CONTEXT.lock().play_music(music);
    #[cfg(target_arch = "wasm32")]
    self::quadsnd::context::music::play_music(music);
}

pub fn get_music_playing() -> Option<Music> {
    #[cfg(not(target_arch = "wasm32"))]
    return self::kira::context::music::MUSIC_CONTEXT.lock().get_music_playing();
    #[cfg(target_arch = "wasm32")]
    return self::quadsnd::context::music::get_current_music();
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