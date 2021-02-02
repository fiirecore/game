use crate::util::Load;


pub mod scene_manager;
pub mod scenes {
	pub mod first_scene;
	// pub mod character_creation_scene;
	// pub mod firsttime_scenes;
	pub mod game_scene;
	pub mod loading_scenes;
	pub mod main_menu_scene;
	pub mod title_scene;
}

pub mod loading_scene_manager;

pub static TITLE_SCENE: usize = 0;
pub static GAME_SCENE: usize = 2;

pub trait Scene: Load {
	
	fn update(&mut self, delta: f32);
	
	fn render(&self);
	
	fn input(&mut self, delta: f32);
	
	fn quit(&mut self);
	
	fn name(&self) -> &str;
	
	fn next_scene(&self) -> usize;
	
}

pub trait LoadingScene {

	fn update(&mut self, delta: f32);

	fn render(&self);

}

// #[async_trait::async_trait]
// pub trait SceneLoad {

// 	fn load(&mut self);

// 	fn on_start(&mut self);

// }