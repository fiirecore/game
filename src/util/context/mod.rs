use std::collections::HashMap;

use oorandom::Rand32 as Random;

use kira::instance::InstanceId;
use kira::manager::AudioManager;
use kira::sound::SoundId;
use piston::Button;

use crate::audio::music::Music;

use self::battle_context::BattleData;

pub mod game_context;

pub mod file_context;
pub mod audio_context;
pub mod battle_context;



pub struct GameContext {

    pub keys: [usize; 8],
    pub fkeys: [usize; 12],

    pub keymap: HashMap<Button, usize>,
    pub fkeymap: HashMap<Button, usize>,
    
    pub random: Random,

    //pub app_console: AppConsole,
    
    pub audio_manager: AudioManager,

    pub music_map: HashMap<Music, SoundId>,
    current_music: Option<InstanceId>,

    pub sound_map: HashMap<u16, SoundId>,
    current_sounds: Vec<InstanceId>,
    
    pub battle_data: Option<BattleData>,

}