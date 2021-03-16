use macroquad::prelude::warn;

pub fn play_music(id: firecore_audio::MusicId) {
    if let Err(err) = firecore_audio::play_music_id(id) {
        warn!("Could not play music id {:x} with error {}", id, err);
    }
}

pub fn play_music_named(music: &str) {
    if let Err(err) = firecore_audio::play_music_named(music) {
        warn!("Could not play music \"{}\" with error {}", music, err);
    }
}