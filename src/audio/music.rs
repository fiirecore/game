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

    pub fn loop_start(&self) -> Option<f64> {
        match self {
            Music::BattleWild => Some(13.15),
            _ => None,
        }
    }

    pub fn file_name(&self) -> &str {
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

}