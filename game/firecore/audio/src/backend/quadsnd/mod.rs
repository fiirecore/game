use macroquad::prelude::info;
use macroquad::prelude::warn;
use self::music::MUSIC_MAP;

pub mod music;

pub async fn bind_world_music() {
    info!("Loading music...");
    for music in firecore_util::music::MUSIC_LIST {
        if !MUSIC_MAP.contains_key(music) {
            match crate::music::included_bytes(&music) {
                Some(bytes) => {
                    read_ogg(*music, bytes);
                }
                None => {
                    let path = String::from("music/") + crate::music::file_name(&music) + ".ogg";
                    match macroquad::prelude::load_file(&path).await {
                        Ok(bytes) => {
                            read_ogg(*music, &bytes);
                        }
                        Err(err) => {
                            warn!("Could not load music file {:?} at {:?} with error {}", music, &path, err);
                        }
                    }
                }
            }
        }
    }
}

pub fn bind_gamefreak() {
    if let Some(bytes) = crate::music::included_bytes(&Music::IntroGamefreak) {
        read_ogg(Music::IntroGamefreak, bytes);
    }
}

fn read_ogg(music: Music, bytes: &[u8]) {
    match quad_snd::decoder::read_ogg(bytes) {
        Ok(sound) => {
            MUSIC_MAP.insert(music, sound);
            info!("Loaded {:?} successfully", music);
        }
        Err(err) => {
            warn!("Could not read bytes for music \"{:?}\" with error {}", music, err);
        }
    }
}