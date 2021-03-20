use self::scenes::SceneState;

pub mod loading;

pub mod manager;
pub mod scenes;

#[async_trait::async_trait(?Send)]
pub trait Scene {

	async fn load(&mut self);

	async fn on_start(&mut self);
	
	fn input(&mut self, delta: f32);
	
	fn update(&mut self, delta: f32);
	
	fn render(&self);

	fn ui(&mut self) {}
	
	fn quit(&mut self);
	
	fn state(&self) -> SceneState;
	
}