use parking_lot::{Mutex, RwLock};
use dashmap::DashMap as HashMap;
use quad_snd::mixer::{Sound, SoundId, SoundMixer, Volume};

lazy_static::lazy_static! {
    pub static ref MIXER: Mutex<SoundMixer> = Mutex::new(SoundMixer::new_ext(Volume(0.2)));
    pub static ref MUSIC_MAP: HashMap<Music, Sound> = HashMap::new();
    static ref CURRENT_MUSIC: RwLock<Option<(Music, SoundId)>> = RwLock::new(None);
}

pub fn play_music(music: Music) {
    let mut mixer = MIXER.lock();
    if let Some(sound_id) = CURRENT_MUSIC.write().take() {
        if sound_id.0 != music {
            mixer.stop(sound_id.1);
        }
    }
    if let Some(sound) = MUSIC_MAP.get(&music) {
        let sound_id = mixer.play(sound.value().clone());
        *CURRENT_MUSIC.write() = Some((music, sound_id));
    }
}

pub fn get_current_music() -> Option<Music> {
    match CURRENT_MUSIC.try_read() {
        Some(music) => {
            return music.as_ref().map(|instance| instance.0);
        }
        None => {
            macroquad::prelude::warn!("Could not read current music!");
            return None;
        }
    }
}