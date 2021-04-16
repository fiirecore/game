#[derive(Clone, Copy)]
pub enum SceneState {

    Continue,
    Scene(Scenes),
    // End,

}

#[derive(Clone, Copy)]
pub enum Scenes {

    // FirstLoad,
    Title,
    MainMenu,

    CharacterCreation,

    // Connect,

    Game,

}

impl Default for Scenes {
    fn default() -> Self {
        #[cfg(not(debug_assertions))] {
            Self::Title
        }
        #[cfg(debug_assertions)] {
            Self::Game
        }
    }
}