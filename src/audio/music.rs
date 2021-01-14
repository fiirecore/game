#[derive(Copy, Clone, Hash, PartialEq, Eq)]
pub enum Music {

    IntroGamefreak,
    IntroPokemon,
    Title,

    ViridianForest,
    MountMoon,
    Route1,
    Route2,
    Route3,
    Route4,
    Gym,
    Fuchsia,
    Pewter,
    Pallet,
    Lavender,
    Celadon,
    Cinnabar,
    Vermilion,

    BattleWild,
    BattleTrainer,
    BattleGym,
    BattleChampion,

}

impl Music {

    pub fn from_int(id: u8) -> Option<Music> {
        match id {
            0x1F => Some(Music::ViridianForest),
            0x13 => Some(Music::Gym),
            0x20 => Some(Music::MountMoon),
            0x23 => Some(Music::Route1),
            0x24 => Some(Music::Route2),
            0x25 => Some(Music::Route3),
            0x26 => Some(Music::Route4),
            0x34 => Some(Music::Fuchsia),
            0x3A => Some(Music::Pewter),
            0x18 => Some(Music::Lavender),
            0x35 => Some(Music::Celadon),
            0x17 => Some(Music::Cinnabar),
            0x39 => Some(Music::Vermilion),
            0x2C => Some(Music::Pallet),
            _ => None,
        }
    }

    pub fn values(&self) -> u8 {
        match *self {
            Music::Gym => 0x13,
            Music::ViridianForest => 0x1F,
            Music::MountMoon => 0x20,
            Music::Route1 => 0x23,
            Music::Route2 => 0x24,
            Music::Route3 => 0x25,
            Music::Route4 => 0x26,
            Music::Pallet => 0x2C,
            Music::Fuchsia => 0x34,
            Music::Pewter => 0x3A,
            Music::Lavender => 0x18,
            Music::Celadon => 0x35,
            Music::Cinnabar => 0x17,
            Music::Vermilion => 0x39,
            _ => 0x24,
        }
    }

    pub fn play_music(music: u8) {
        music::play_music(&Music::from_int(music).unwrap_or(Music::Pallet), music::Repeat::Forever);
    }

}