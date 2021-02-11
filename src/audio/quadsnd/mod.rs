use macroquad::prelude::info;
use enum_iterator::IntoEnumIterator;
use macroquad::prelude::warn;
use self::context::music::MUSIC_MAP;
use super::music::Music;

pub mod context;

pub async fn bind_world_music() {
    info!("Loading music...");
    for music in Music::into_enum_iter() {
        //if !music_map.contains_key(&music) {
            match music.included_bytes() {
                Some(bytes) => {
                    match quad_snd::decoder::read_ogg(bytes) {
                        Ok(sound) => {
                            MUSIC_MAP.write().insert(music, sound);
                            info!("Loaded {:?} successfully", music);
                        }
                        Err(err) => {
                            warn!("Problem decoding {:?}'s bytes in executable with error {}", &music, err);
                        }
                    } 
                }
                None => {
                    let path = std::path::PathBuf::from("music").join(music.to_string() + ".ogg");
                    match macroquad::prelude::load_file(&path).await {
                        Ok(bytes) => {
                            match quad_snd::decoder::read_ogg(&bytes) {
                                Ok(sound) => {
                                    MUSIC_MAP.write().insert(music, sound);
                                    info!("Loaded {:?} successfully", music);
                                }
                                Err(err) => {
                                    warn!("Problem decoding {:?}'s bytes in executable with error {}", &music, err);
                                }
                            } 
                        }
                        Err(err) => {
                            warn!("Could not load music file {:?} at {:?} with error {}", music, &path, err);
                        }
                    }
                }
            }  
        //}
    }
}

pub fn bind_gamefreak() {
    if let Some(bytes) = Music::IntroGamefreak.included_bytes() {
        match quad_snd::decoder::read_ogg(bytes) {
            Ok(sound) => {
                self::context::music::MUSIC_MAP.write().insert(Music::IntroGamefreak, sound);
            }
            Err(err) => {
                warn!("Could not read bytes for gamefreak intro with error {}", err);
            }
        }
        
    }
     
}