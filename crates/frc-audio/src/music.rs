#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq, enum_iterator::IntoEnumIterator, serde::Deserialize, serde::Serialize)]
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

    EncounterBoy,
    EncounterGirl,

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

    pub const fn loop_start(&self) -> Option<f64> {
        match self {
            Music::BattleWild => Some(13.15),
            _ => None,
        }
    }

    pub const fn file_name(&self) -> &str {
        match self {

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

            
            Music::EncounterBoy => "encounter_boy",
            Music::EncounterGirl => "encounter_girl",

            Music::BattleWild => "vs_wild",
            Music::BattleTrainer => "vs_trainer",
            Music::BattleGym => "vs_gym",
            
        }
    }

    pub const fn included_bytes(&self) -> Option<&[u8]> { // To - do: Load dynamically from assets folder instead of specifying this
        let mut bytes: Option<&[u8]> = match self {
            Music::IntroGamefreak => Some(include_bytes!("../music/gamefreak.ogg")),
            Music::Title => Some(include_bytes!("../music/title.ogg")),
            Music::Pallet => Some(include_bytes!("../music/pallet.ogg")),
            Music::EncounterBoy => Some(include_bytes!("../music/encounter_boy.ogg")),
            Music::BattleWild => Some(include_bytes!("../music/vs_wild.ogg")),
            Music::BattleTrainer => Some(include_bytes!("../music/vs_trainer.ogg")),
            Music::BattleGym => Some(include_bytes!("../music/vs_gym.ogg")),
            _ => None,
        };
        #[cfg(debug_assertions)] {
            if bytes.is_none() {
                bytes = match self {
                    Music::Pewter => Some(include_bytes!("../../../music/pewter.ogg")),
                    Music::Fuchsia => Some(include_bytes!("../../../music/fuchsia.ogg")),
                    Music::Lavender => Some(include_bytes!("../../../music/lavender.ogg")),
                    Music::Celadon => Some(include_bytes!("../../../music/celadon.ogg")),
                    Music::Cinnabar => Some(include_bytes!("../../../music/cinnabar.ogg")),
                    Music::Vermilion => Some(include_bytes!("../../../music/vermilion.ogg")),
                    Music::Route1 => Some(include_bytes!("../../../music/route1.ogg")),
                    Music::Route2 => Some(include_bytes!("../../../music/route2.ogg")),
                    Music::Route3 => Some(include_bytes!("../../../music/route3.ogg")),
                    Music::Route4 => Some(include_bytes!("../../../music/route3.ogg")),
                    Music::ViridianForest => Some(include_bytes!("../../../music/viridian_forest.ogg")),
                    Music::MountMoon => Some(include_bytes!("../../../music/mt_moon.ogg")),
                    Music::Gym => Some(include_bytes!("../../../music/gym.ogg")),
                    Music::EncounterGirl => Some(include_bytes!("../../../music/encounter_girl.ogg")),
                    _ => None,
                }
            }
        }
        
        bytes
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
                macroquad::prelude::warn!("Could not get music with id 0x{:X}!", id);
                return Music::default();
            },
        }
    }
}