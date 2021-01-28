use self::battle_context::BattleData;

pub mod file_context;
// pub mod audio_context;
pub mod battle_context;

#[deprecated(since = "0.2.0", note = "Not needed anymore")]
pub struct GameContext {

    // pub fkeys: [usize; 12],
    // pub fkeymap: AHashMap<macroquad::prelude::KeyCode, usize>,
    
    // pub audio_context: AudioContext,

    // pub audio_manager: AudioManager,

    // pub music_map: AHashMap<Music, SoundId>,
    // current_music: Option<InstanceId>,

    // pub sound_map: AHashMap<u16, SoundId>,
    // current_sounds: Vec<InstanceId>,

}

impl GameContext {

    pub fn new() -> GameContext {

        GameContext {

            // fkeys: [0; 12],
            // fkeymap: AHashMap::new(),

            // audio_context: super::audio_context::AudioContext::new(),

            // audio_manager: kira::manager::AudioManager::new(kira::manager::AudioManagerSettings::default()).unwrap(),
            
            // music_map: AHashMap::new(),
            // current_music: None,

            // sound_map: AHashMap::new(),
            // current_sounds: Vec::new(),

            battle_data: None,

        }
    }

}