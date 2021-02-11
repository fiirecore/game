use parking_lot::{Mutex, RwLock};
use ahash::AHashMap as HashMap;
use quad_snd::mixer::{Sound, SoundId, SoundMixer, MixerInternal, Volume};

use crate::audio::music::Music;

lazy_static::lazy_static! {
    pub static ref MIXER: Mutex<SoundMixer> = Mutex::new({
        let mut driver = quad_snd::SoundDriver::new(Box::new(MixerInternal {
            sample_rate: 22050.0,
            volume: Volume(0.2),
            ..Default::default()
        }));
        driver.start();
        SoundMixer { driver, uid: 0 }
    });
    pub static ref MUSIC_MAP: RwLock<HashMap<Music, Sound>> = RwLock::new(HashMap::new());
    static ref CURRENT_MUSIC: Mutex<Option<(Music, SoundId)>> = Mutex::new(None);
}

pub fn play_music(music: Music) {
    let mut mixer = MIXER.lock();
    if let Some(sound_id) = CURRENT_MUSIC.lock().take() {
        if sound_id.0 != music {
            mixer.stop(sound_id.1);
        }
    }
    if let Some(sound) = MUSIC_MAP.read().get(&music) {
        let sound_id = mixer.play(sound.clone());
        *CURRENT_MUSIC.lock() = Some((music, sound_id));
    }
}

pub fn get_current_music() -> Option<Music> {
    return CURRENT_MUSIC.lock().map(|music| music.0);
}