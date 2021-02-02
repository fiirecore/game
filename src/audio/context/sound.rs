use ahash::AHashMap as HashMap;
use kira::instance::handle::InstanceHandle;
use kira::sound::handle::SoundHandle;

use crate::audio::sound::Sound;

pub struct SoundContext {

    sound_map: HashMap<Sound, SoundHandle>,
    current_sounds: Vec<InstanceHandle>,

}