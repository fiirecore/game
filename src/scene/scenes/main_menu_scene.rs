use crate::gui::GuiComponent;
use crate::gui::background::Background;
use crate::gui::button::BasicButton;
use crate::io::data::player::PlayerData;
use frc_input as input;
use crate::scene::Scene;
use crate::util::Entity;
use super::SceneState;
use super::Scenes;

pub struct MainMenuScene {
	state: SceneState,
	continue_background: Background,
	continue_button: BasicButton,
	// new_game_background: Background,
}

impl MainMenuScene {

	pub fn new() -> MainMenuScene {
		let panel_x = 17.0;
		let panel_y = 1.0;
		MainMenuScene {
			state: SceneState::Continue,
			continue_background: Background::new(crate::util::graphics::texture::byte_texture(include_bytes!("../../../build/assets/menu_button.png")), panel_x, panel_y),
			continue_button: BasicButton::new("Continue", 1, 10.0, 10.0, panel_x, panel_y),
		}
	}

}

#[async_trait::async_trait(?Send)]
impl Scene for MainMenuScene {

	// have normal main menu + video settings + controls + exit

	async fn load(&mut self) {

	}

	fn loaded(&self) -> bool {
		true
	}

	fn on_start(&mut self) {
		self.continue_button.spawn();
	}
	
	fn update(&mut self, delta: f32) {

	}
	
	fn render(&self) {
		self.continue_background.render();
		self.continue_button.render();
	}
	
	fn input(&mut self, delta: f32) {
		if input::pressed(input::Control::A) {
			self.state = SceneState::Scene(Scenes::GameScene);
		}
	}
	
	fn quit(&mut self) {
		PlayerData::select_data("player".to_string());
	}
	
	fn state(&self) -> SceneState {
		self.state
	}
	
}