#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq, enum_iterator::IntoEnumIterator, serde::Deserialize)]
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

    #[cfg(any(feature = "audio", feature = "webaudio"))]
    pub fn included_bytes(&self) -> Option<&[u8]> { // To - do: Load dynamically from assets folder instead of specifying this
        match self {
            Music::IntroGamefreak => Some(include_bytes!("../../build/assets/music/gamefreak.ogg")),
            Music::Title => Some(include_bytes!("../../build/assets/music/title.ogg")),
            Music::Pallet => Some(include_bytes!("../../build/assets/music/pallet.ogg")),
            Music::EncounterBoy => Some(include_bytes!("../../build/assets/music/encounter_boy.ogg")),
            Music::BattleWild => Some(include_bytes!("../../build/assets/music/vs_wild.ogg")),
            Music::BattleTrainer => Some(include_bytes!("../../build/assets/music/vs_trainer.ogg")),
            Music::BattleGym => Some(include_bytes!("../../build/assets/music/vs_gym.ogg")),
            _ => None,
        }
    }

}

//#[deprecated]
impl std::fmt::Display for Music {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {

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

            Music::BattleWild => "vs_wild",
            Music::BattleTrainer => "vs_trainer",
            Music::BattleGym => "vs_gym",

        })
    }
}