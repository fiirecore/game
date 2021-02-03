use self::music::Music;

pub mod music;
pub mod sound;
#[cfg(feature = "audio")]
pub mod loader;
#[cfg(feature = "audio")]
pub mod context;

pub fn play_music(music: Music) {
    macroquad::prelude::debug!("Playing {:?}", music);
    #[cfg(feature = "audio")]
    self::context::music::MUSIC_CONTEXT.lock().play_music(music);
}

pub fn get_music_playing() -> Option<Music> {
    #[cfg(feature = "audio")]
    return self::context::music::MUSIC_CONTEXT.lock().get_music_playing();
    #[cfg(not(feature = "audio"))]
    return None;
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