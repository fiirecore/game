use self::scenes::SceneState;

pub mod loading;

pub mod manager;
pub mod scenes;

pub trait Scene {

	fn new() -> Self where Self: Sized;

	fn on_start(&mut self);
	
	fn input(&mut self, delta: f32);
	
	fn update(&mut self, delta: f32);
	
	fn render(&self);
	
	fn quit(&mut self);
	
	fn state(&self) -> SceneState;
	
}