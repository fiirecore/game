pub mod context;
pub mod music;
pub mod sound;

pub struct MusicHandle(sdl2::mixer::Music<'static>);

unsafe impl Send for MusicHandle {}
unsafe impl Sync for MusicHandle {}

pub struct SoundHandle(sdl2::mixer::Chunk);

unsafe impl Send for SoundHandle {}
unsafe impl Sync for SoundHandle {}