use kira::sound::SoundSettings;

#[derive(Copy, Clone, Hash, PartialEq, Eq, enum_iterator::IntoEnumIterator)]
pub enum Music {

    IntroGamefreak,
    // IntroPokemon,
    Title, // 45.010

    Pallet, // 43.640
    Pewter,
    Fuchsia,
    Lavender,
    Celadon,
    Cinnabar,
    Vermilion,

    Route1, // 25.090
    Route2,
    Route3,
    Route4,

    ViridianForest,
    MountMoon,
    Gym,

    BattleWild, // 44.480
    BattleTrainer, // 1:41.870
    BattleGym, // 56.780
    // BattleChampion,

}

impl Default for Music {
    fn default() -> Self {
        Music::Pallet
    }
}

impl Music {

    pub fn included_bytes(&self) -> Option<&[u8]> {
        match *self {
            Music::IntroGamefreak => Some(include_bytes!("../../assets/music/gamefreak.ogg")),
            Music::Title => Some(include_bytes!("../../assets/music/title.ogg")),
            Music::Pallet => Some(include_bytes!("../../assets/music/pallet.ogg")),
            Music::BattleWild => Some(include_bytes!("../../assets/music/vs_wild.ogg")),
            _ => None,
        }
    }

    pub fn loop_start(&self) -> Option<f64> {
        match *self {
            Music::BattleWild => Some(13.15),
            _ => None,
        }
    }

    pub fn settings(&self) -> SoundSettings {
        SoundSettings {
            default_loop_start: self.loop_start(),
            ..Default::default()
        }
    }

}

impl From<u8> for Music {
    fn from(id: u8) -> Self {
        match id {
            0x1F => Music::ViridianForest,
            0x13 => Music::Gym,
            0x20 => Music::MountMoon,
            0x23 => Music::Route1,
            0x24 => Music::Route2,
            0x25 => Music::Route3,
            0x26 => Music::Route4,
            0x34 => Music::Fuchsia,
            0x3A => Music::Pewter,
            0x18 => Music::Lavender,
            0x35 => Music::Celadon,
            0x17 => Music::Cinnabar,
            0x39 => Music::Vermilion,
            0x2C => Music::Pallet,
            _ => {
                macroquad::prelude::warn!("Could not get music with id #{}!", id);
                return Music::default();
            },
        }
    }
}

impl std::fmt::Display for Music {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match *self {

            Music::IntroGamefreak => "gamefreak",
            Music::Title => "title",

            Music::Pallet => "pallet",
            Music::Pewter => "pewter",
            Music::Fuchsia => "fuchsia",
            Music::Lavender => "lavender",
            Music::Celadon => "celadon",
            Music::Cinnabar => "cinnabar",
            Music::Vermilion => "vermilion",

            Music::Route1 => "route1",
            Music::Route2 => "route2",
            Music::Route3 => "route3",
            Music::Route4 => "route4",

            Music::Gym => "gym",
            Music::ViridianForest => "viridian_forest",
            Music::MountMoon => "mt_moon",

            Music::BattleWild => "vs_wild",
            Music::BattleTrainer => "vs_trainer",
            Music::BattleGym => "vs_gym",

        })
    }
}