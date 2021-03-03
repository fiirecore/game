use dashmap::DashMap as DashMap;
use kira::instance::handle::InstanceHandle;
use kira::sound::handle::SoundHandle;

use frc_audio::sound::Sound;

pub struct SoundContext {

    sound_map: HashMap<Sound, SoundHandle>,
    current_sounds: Vec<InstanceHandle>,

}