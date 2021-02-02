use self::context::music::MUSIC_CONTEXT;

pub mod music;
pub mod sound;
pub mod loader;
pub mod context;

pub fn play_music(music: self::music::Music) {
    MUSIC_CONTEXT.lock().play_music(music);
}

pub fn is_music_playing() -> bool {
    MUSIC_CONTEXT.lock().is_music_playing()
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