use std::borrow::Cow;

use game::{
	gui::{Button, ButtonBase},
	input::{pressed, Control},
	storage::{saves, player::PlayerSaves},
	text::TextColor,
	graphics::{draw_text_left, draw_rectangle, draw_rectangle_lines, DARKBLUE},
	tetra::{
		Context, State, Result,
		math::Vec2,
		input,
	},
};

use super::{MenuState, MenuStateAction, MenuStates};


// have normal main menu + video settings + controls + exit

pub struct MainMenuState {

	action: Option<MenuStateAction>,

	cursor: usize,

	saves: Vec<Button>,

	delete: bool,

	last_mouse_pos: Vec2<f32>,

	new_game: ButtonBase,
	delete_button: ButtonBase,

	scaler: Vec2<f32>,

}

impl MainMenuState {

	const GAP: f32 = 35.0;

	pub fn new(ctx: &mut Context, scaler: Vec2<f32>) -> Self {
		Self {
			action: None,
			cursor: Default::default(),
			saves: Vec::new(),
			delete: false,
			last_mouse_pos: Default::default(),
			new_game: ButtonBase::new(ctx, Vec2::new(206.0, 30.0), Cow::Borrowed("New Game")),
			delete_button: ButtonBase::new(ctx, Vec2::new(206.0, 30.0), Cow::Borrowed("Play/Delete")),
			scaler,
		}
	}

	fn update_saves(ctx: &mut Context, list: &mut Vec<Button>, saves: &'static PlayerSaves) {
		*list = saves.saves.iter().enumerate().map(|(index, save)| Button::new(ctx, Vec2::new(20.0, 5.0 + index as f32 * Self::GAP), Vec2::new(206.0, 30.0), Cow::Borrowed(save.as_ref().map(|save| save.name.as_str()).unwrap_or("ERROR")))).collect();
	}

}

impl State for MainMenuState {

	fn begin(&mut self, ctx: &mut Context) -> Result {
		self.cursor = Default::default();
		self.delete = false;
		Self::update_saves(ctx, &mut self.saves, saves());
		Ok(())
	}
	
	fn update(&mut self, ctx: &mut Context) -> Result {

		let mouse_pos = input::get_mouse_position(ctx) * self.scaler;

		let last = if self.last_mouse_pos != mouse_pos {
			self.last_mouse_pos = mouse_pos;
			true
		} else {
			false
		};

		for (index, button) in self.saves.iter_mut().enumerate() {
			let (click, mouse) = button.update(ctx, last.then(|| mouse_pos), self.cursor == index);
			if mouse {
				self.cursor = index;
			}
			if click {
				let saves = saves();
				if self.delete {
					if saves.delete(index) {
						// if index >= self.cursor {
						// 	self.cursor -= 1;
						// }
						Self::update_saves(ctx, &mut self.saves, saves);
						break;
					};
				} else {
					saves.select(index);
					self.action = Some(MenuStateAction::StartGame);
				}
			}
		}

		let new_game_pos = self.saves.len();

		{

			let (click, mouse) = self.new_game.update(ctx, &Vec2::new(20.0, 5.0 + new_game_pos as f32 * Self::GAP), last.then(|| mouse_pos), self.cursor == new_game_pos);
	
			if mouse {
				self.cursor = new_game_pos;
			}
	
			if click {
				self.action = Some(MenuStateAction::Goto(MenuStates::CharacterCreation));
			}

		}

		let delete_pos = new_game_pos + 1;

		{

			let (click, mouse) = self.delete_button.update(ctx, &Vec2::new(20.0, 5.0 + delete_pos as f32 * Self::GAP), last.then(|| mouse_pos), self.cursor == delete_pos);
	
			if mouse {
				self.cursor = delete_pos;
			}
	
			if click {
				self.delete = !self.delete;
			}

		}

		if pressed(ctx, Control::B) {
			self.action = Some(MenuStateAction::Goto(MenuStates::Title));
		}

		if pressed(ctx, Control::Up) && self.cursor > 0 {
			self.cursor -= 1;
		}

		if pressed(ctx, Control::Down) && self.cursor <= self.saves.len() {
			self.cursor += 1;
		}

		Ok(())
	}
	
	fn draw(&mut self, ctx: &mut Context) -> Result {

		draw_rectangle(ctx, 0.0, 0.0, game::util::WIDTH, game::util::HEIGHT, DARKBLUE);

		for save in self.saves.iter() {
			save.draw(ctx);
			// self.button.draw(ctx, 20.0, y, 206.0, 30.0);
			// draw_text_left(ctx, &1, save, &TextColor::Black, 31.0, y + 5.0);
		}

		let saves_len = self.saves.len() as f32;

		{
			let y = 5.0 + saves_len * Self::GAP;
			self.new_game.draw(ctx, Vec2::new(20.0, y));
		// 	draw_text_left(ctx, &1, "New Game", &TextColor::Black, 31.0, y + 5.0);
		}

		{
			let y = 5.0 + (saves_len + 1.0) * Self::GAP;
			self.delete_button.draw(ctx, Vec2::new(20.0, y));
		// 	draw_text_left(ctx, &1, &TextColor::Black, 31.0, y + 5.0);
		}

		draw_rectangle_lines(ctx, 20.0, 5.0 + self.cursor as f32 * Self::GAP, 206.0, 30.0, 2.0, game::graphics::RED);

		draw_text_left(ctx, &1, if self.delete { "Delete Mode: ON" } else { "Delete Mode: OFF" }, &TextColor::Black, 5.0, 145.0);

		Ok(())
	}
}

impl MenuState for MainMenuState {
    fn next(&mut self) -> &mut Option<MenuStateAction> {
        &mut self.action
    }
}