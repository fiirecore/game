pub mod manager;

pub mod copyright;
pub mod gamefreak;

pub enum LoadingState {

    Continue,
    Scene(LoadingScenes),
    End,

}

#[derive(Clone, Copy)]
pub enum LoadingScenes {

    Copyright,
    Gamefreak,

}

impl Default for LoadingScenes {
    fn default() -> Self {
        Self::Copyright
    }
}

pub trait LoadingScene {

    fn on_start(&mut self);

    fn update(&mut self, delta: f32);

    fn render(&self);

    fn state(&self) -> &LoadingState;

}