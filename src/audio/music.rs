use super::Music;

lazy_static::lazy_static! {
    pub static ref GAMEFREAK_MUSIC: parking_lot::Mutex<Option<kira::sound::handle::SoundHandle>> = parking_lot::Mutex::new(None);
}