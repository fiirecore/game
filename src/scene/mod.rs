pub mod scene_manager;
pub mod loading_scene_manager;

pub mod scenes;

pub trait Scene {

	fn on_start(&mut self);
	
	fn update(&mut self, delta: f32);
	
	fn render(&self);
	
	fn input(&mut self, delta: f32);
	
	fn quit(&mut self);
	
	// fn name(&self) -> &str;
	
	fn next_scene(&self) -> Option<scenes::Scenes>;
	
}