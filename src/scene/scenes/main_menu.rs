use firecore_util::text::TextColor;
use macroquad::prelude::Texture2D;
use macroquad::prelude::{RED, DARKBLUE};
use macroquad::prelude::collections::storage::{get, get_mut};

use firecore_input::{pressed, Control};
use macroquad::prelude::draw_rectangle;
use macroquad::prelude::draw_rectangle_lines;
use macroquad::prelude::warn;

use firecore_data::player::PlayerSaves;
use crate::scene::Scene;
use crate::util::graphics::{byte_texture, draw, draw_text_left};

use super::SceneState;
use super::Scenes;

const GAP: f32 = 35.0;

pub struct MainMenuScene {

	state: SceneState,

	button: Texture2D,
	cursor: usize,

	saves: Vec<String>,

	delete: bool,
	// name: String,

}

impl MainMenuScene {

	fn update_saves(&mut self, saves: &PlayerSaves) {
		self.saves = saves.name_list().into_iter().map(|name| name.clone()).collect();
	}

}

// have normal main menu + video settings + controls + exit

impl Scene for MainMenuScene {

	fn new() -> MainMenuScene {
		MainMenuScene {
			state: SceneState::Continue,
			button: byte_texture(include_bytes!("../../../build/assets/menu_button.png")),
			cursor: 0,
			saves: Vec::new(),
			delete: false,
			// name: String::new(),
		}
	}

	fn on_start(&mut self) {
		// set_camera(crate::util::window_camera());
		self.cursor = 0;
		self.delete = false;
		if let Some(saves) = get::<PlayerSaves>() {
			self.update_saves(&saves);
		}
	}
	
	fn update(&mut self, _delta: f32) {}
	
	fn render(&self) {

		draw_rectangle(0.0, 0.0, crate::WIDTH, crate::HEIGHT, DARKBLUE);

		for (index, save) in self.saves.iter().enumerate() {
			let y = 5.0 + index as f32 * GAP;
			draw(self.button, 20.0, y);
			draw_text_left(1, save, TextColor::Black, 31.0, y + 5.0);
		}

		let saves_len = self.saves.len() as f32;

		{
			let y = 5.0 + saves_len * GAP;
			draw(self.button, 20.0, y);
			draw_text_left(1, "New Game", TextColor::Black, 31.0, y + 5.0);
		}

		{
			let y = 5.0 + (saves_len + 1.0) * GAP;
			draw(self.button, 20.0, y);
			draw_text_left(1, if self.delete { "Play" } else { "Delete" }, TextColor::Black, 31.0, y + 5.0);
		}

		draw_rectangle_lines(20.0, 5.0 + self.cursor as f32 * GAP, 206.0, 30.0, 2.0, RED);

		draw_text_left(1, if self.delete { "Delete Mode: ON" } else { "Delete Mode: OFF" }, TextColor::Black, 5.0, 145.0);

	}
	
	fn input(&mut self, _delta: f32) {
		if pressed(Control::A) {
			if self.cursor == self.saves.len() {
				self.state = SceneState::Scene(Scenes::CharacterCreation);
				// saves.select_new(&firecore_data::player::default_name());
			} else if self.cursor == self.saves.len() + 1 {
				self.delete = !self.delete;
			} else {
				if let Some(mut saves) = get_mut::<PlayerSaves>() {
					if self.delete {
						if saves.delete(self.cursor) {
							self.cursor -= 1;
							self.update_saves(&saves);
						};
					} else {
						saves.select(self.cursor);
						self.state = SceneState::Scene(Scenes::Game);
					}					
				} else {
					warn!("Could not get player save data!");
				}
			}
					
		}

		if pressed(Control::B) {
			self.state = SceneState::Scene(Scenes::Title);
		}

		if pressed(Control::Up) && self.cursor > 0 {
			self.cursor -= 1;
		}

		if pressed(Control::Down) && self.cursor <= self.saves.len() {
			self.cursor += 1;
		}

	}
	
	fn quit(&mut self) {
		self.state = SceneState::Continue;
	}
	
	fn state(&self) -> SceneState {
		self.state
	}
	
}