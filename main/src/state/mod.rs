pub mod manager;

pub mod loading;
pub mod menu;
pub mod game;

// put state trait here that other state traits derive

pub trait State {

	fn new() -> Self where Self: Sized;

	fn on_start(&mut self);

	fn update(&mut self, delta: f32) -> Option<States>;

	fn render(&self);

	fn quit(&mut self);

}

pub enum States {

	// Loading,
	Menu,
	Game,

}

impl Default for States {
    fn default() -> Self {
        #[cfg(not(debug_assertions))] {
            Self::Menu
        }
        #[cfg(debug_assertions)] {
            Self::Game
        }
    }
}