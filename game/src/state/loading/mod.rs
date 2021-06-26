pub mod manager;

pub mod copyright;
pub mod gamefreak;
pub mod pokemon;

pub async fn load_coroutine() {

    use game::macroquad::prelude::{info, get_frame_time, clear_background, BLACK, next_frame};
    use game::input::{pressed, Control};

    info!("Starting loading scene coroutine");

    let mut manager = manager::LoadingSceneManager::new();

    while !manager.finished {

        if game::init::LOADING_FINISHED.load(std::sync::atomic::Ordering::Relaxed) {
            if pressed(Control::A) {
                manager.finished = true;
            }
        }

        manager.update(get_frame_time().min(0.5));
        clear_background(BLACK);
        manager.render();
        next_frame().await;
    }
}

#[derive(Clone, Copy)]
pub enum LoadingState {

    Continue,
    Scene(LoadingScenes),
    End,

}

#[derive(Clone, Copy)]
pub enum LoadingScenes {

    Copyright,
    Gamefreak,
    Pokemon,

}

impl Default for LoadingScenes {
    fn default() -> Self {
        Self::Copyright
    }
}

pub trait LoadingScene {

    fn new() -> Self where Self: Sized;

    fn on_start(&mut self);

    fn update(&mut self, delta: f32);

    fn render(&self);

    fn state(&self) -> LoadingState;

}