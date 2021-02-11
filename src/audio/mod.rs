use self::music::Music;

pub mod music;
pub mod sound;
#[cfg(feature = "audio")]
pub mod kira;
#[cfg(feature = "webaudio")]
pub mod quadsnd;

pub async fn bind_world_music() {
    #[cfg(feature = "audio")]
    self::kira::bind_world_music();
    #[cfg(feature = "webaudio")]
    self::quadsnd::bind_world_music().await;
}

pub fn play_music(music: Music) {
    macroquad::prelude::debug!("Playing {:?}", music);
    #[cfg(feature = "audio")]
    self::kira::context::music::MUSIC_CONTEXT.lock().play_music(music);
    #[cfg(feature = "webaudio")]
    self::quadsnd::context::music::play_music(music);
}

pub fn get_music_playing() -> Option<Music> {
    #[cfg(feature = "audio")]
    return self::kira::context::music::MUSIC_CONTEXT.lock().get_music_playing();
    #[cfg(feature = "webaudio")]
    return self::quadsnd::context::music::get_current_music();
    #[cfg(not(any(feature = "audio", feature = "webaudio")))]
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