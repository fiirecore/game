use self::scenes::SceneState;

pub mod loading;

pub mod manager;
pub mod scenes;

pub trait Scene {

	fn on_start(&mut self);
	
	fn update(&mut self, delta: f32);
	
	fn render(&self);
	
	fn input(&mut self, delta: f32);
	
	fn quit(&mut self);
	
	fn state(&self) -> SceneState;
	
}