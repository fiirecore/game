use macroquad::camera::set_camera;
use macroquad::prelude::Rect;
use macroquad::prelude::collections::storage::{get, get_mut};
use macroquad::ui::widgets::InputText;
use macroquad::camera::Camera2D;
use macroquad::prelude::info;
use macroquad::prelude::screen_height;
use macroquad::prelude::screen_width;
use macroquad::prelude::vec2;

use crate::data::player::list::PlayerSaves;
use crate::scene::Scene;

use super::SceneState;
use super::Scenes;

use macroquad::ui::{
    hash, root_ui,
    widgets::{Window, Group}
};

pub struct MainMenuScene {
	state: SceneState,
	saves: Vec<String>,
	name: String,

}

impl MainMenuScene {

	pub fn new() -> MainMenuScene {
		MainMenuScene {
			state: SceneState::Continue,
			saves: Vec::new(),
			name: String::new(),
		}
	}

}

#[async_trait::async_trait(?Send)]
impl Scene for MainMenuScene {

	// have normal main menu + video settings + controls + exit

	async fn load(&mut self) {
		if let Some(saves) = get::<PlayerSaves>() {
			self.saves = saves.name_list().into_iter().map(|name| name.clone()).collect();
		}
	}

	async fn on_start(&mut self) {
		set_camera(Camera2D::from_display_rect(Rect::new(0.0, 0.0, screen_width(), screen_height())));
	}
	
	fn update(&mut self, _delta: f32) {}
	
	fn render(&self) {}

	fn ui(&mut self) {

		Window::new(hash!(), vec2(0.0, 0.0), vec2(screen_width(), screen_height()))
			.label("Player Saves")
			.movable(false)
			.titlebar(true)
			.close_button(false)
			.ui(&mut *root_ui(), |ui| {
				for i in 0..self.saves.len() {
					Group::new(hash!("sav", i), vec2(220.0, 50.0)).ui(ui, |ui| {
						ui.label(vec2(5.0, 5.0), &self.saves[i]);
						if ui.button(vec2(80.0, 5.0), "Play") {
							if let Some(mut saves) = get_mut::<PlayerSaves>() {
								saves.select(i);
								self.state = SceneState::Scene(Scenes::GameScene);
							}							
						}
					});
				}
				Group::new(hash!("new"), vec2(220.0, 50.0)).ui(ui, |ui| {
					ui.label(vec2(5.0, 30.0), "New Game");
					InputText::new(0).label("Name").ui(ui, &mut self.name);
					if ui.button(vec2(150.0, 5.0), "Play") {
						if !self.name.is_empty() {
							if let Some(mut saves) = get_mut::<PlayerSaves>() {
								saves.select_new(&self.name);
								self.state = SceneState::Scene(Scenes::GameScene);
							}
						} else {
							info!("Could not create new game because player name is empty!");
						}
					}
				});
			}
		);

		
	}
	
	fn input(&mut self, _delta: f32) {}
	
	fn quit(&mut self) {
		set_camera(Camera2D::from_display_rect(crate::CAMERA_SIZE));
	}
	
	fn state(&self) -> SceneState {
		self.state
	}
	
}