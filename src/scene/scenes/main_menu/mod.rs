use ahash::AHashSet as HashSet;
use macroquad::ui::widgets::InputText;

use crate::io::data::player::PlayerData;
use firecore_input as input;
use macroquad::camera::Camera2D;
use macroquad::prelude::info;
use macroquad::prelude::screen_height;
use macroquad::prelude::screen_width;
use macroquad::prelude::vec2;
use crate::scene::Scene;
use self::save_list::SaveList;

use super::SceneState;
use super::Scenes;

use macroquad::ui::{
    hash, root_ui,
    widgets::{Window, Group}
};

pub mod save_list;

pub struct MainMenuScene {
	state: SceneState,
	
	// continue_background: Background,
	// continue_button: BasicButton,
    // window: Window,
    saves: HashSet<String>,
	name: String,
	// new_game_background: Background,
}

impl MainMenuScene {

	pub fn new() -> MainMenuScene {
		// let panel_x = 17.0;
		// let panel_y = 1.0;
		MainMenuScene {
			state: SceneState::Continue,
			// continue_background: Background::new(crate::util::graphics::texture::byte_texture(include_bytes!("../../../build/assets/menu_button.png")), panel_x, panel_y),
			// continue_button: BasicButton::new("Continue", 1, 10.0, 10.0, panel_x, panel_y),
			// window: ,
            saves: HashSet::new(),
			name: String::new(),
		}
	}

}

#[async_trait::async_trait(?Send)]
impl Scene for MainMenuScene {

	// have normal main menu + video settings + controls + exit

	async fn load(&mut self) {
		self.saves = SaveList::get().players;
	}

	async fn on_start(&mut self) {
		macroquad::camera::set_camera(Camera2D::from_display_rect(macroquad::prelude::Rect::new(0.0, 0.0, screen_width(), screen_height())));
		// self.continue_button.spawn();
	}
	
	fn update(&mut self, _delta: f32) {

		

		Window::new(hash!(), vec2(0.0, 0.0), vec2(screen_width(), screen_height()))
			.label("Player Saves")
			.movable(false)
			.titlebar(true)
			.close_button(false)
			.ui(&mut *root_ui(), |ui| {
				for i in 0..self.saves.len() {
					Group::new(hash!("sav", i), vec2(220.0, 50.0)).ui(ui, |ui| {
						let name = self.saves.iter().nth(i).unwrap();
						ui.label(vec2(5.0, 5.0), name);
						if ui.button(vec2(80.0, 5.0), "Play") {
							PlayerData::select_data(name);
							self.state = SceneState::Scene(Scenes::GameScene);
						}
					});
				}
				Group::new(hash!("new"), vec2(220.0, 50.0)).ui(ui, |ui| {
					ui.label(vec2(5.0, 30.0), "New Game");
					InputText::new(0).label("Name").ui(ui, &mut self.name);
					if ui.button(vec2(150.0, 5.0), "Play") {
						if !self.name.is_empty() {
							PlayerData::select_data(&self.name);
							self.state = SceneState::Scene(Scenes::GameScene);
						} else {
							info!("Could not create new game because player name is empty!");
						}
					}
					// if Button::new("Play").position(vec2(50.0, 5.0)).size(vec2(60.0, 20.0)).ui(ui) {
					// 	info!("Unimplemented!");
					// }
				});
			}
		);
	}
	
	fn render(&self) {
		// self.continue_background.render();
		// self.continue_button.render();
		

	}
	
	fn input(&mut self, _delta: f32) {
		if input::pressed(input::Control::A) {
			self.state = SceneState::Scene(Scenes::GameScene);
		}
	}
	
	fn quit(&mut self) {
		macroquad::camera::set_camera(Camera2D::from_display_rect(macroquad::prelude::Rect::new(0.0, 0.0, crate::BASE_WIDTH as _, crate::BASE_HEIGHT as _)));
		// PlayerData::select_data("player".to_string());
	}
	
	fn state(&self) -> SceneState {
		self.state
	}
	
}