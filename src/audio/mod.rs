use std::ops::DerefMut;
use macroquad::prelude::collections::storage::get_mut;
use crate::util::audio::AudioContext;

pub mod music;
pub mod sound;
pub mod loader;

pub fn play_music(music: self::music::Music) {
    if let Some(mut audio_context) = get_mut::<AudioContext>() {
        audio_context.deref_mut().play_music(music);
    }
}

pub fn is_music_playing() -> bool {
    if let Some(mut audio_context) = get_mut::<AudioContext>() {
        return audio_context.deref_mut().is_music_playing();
    } else {
        return false;
    }
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