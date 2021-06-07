use game::{
	gui::Panel,
	input::{pressed, Control},
	storage::{PLAYER_SAVES, player::PlayerSaves},
	text::TextColor,
	graphics::{draw_text_left, draw_rectangle, draw_rectangle_lines, DARKBLUE},
	tetra::{Context, State, Result},
	log::warn,
};

use super::{MenuState, MenuStateAction, MenuStates};


// have normal main menu + video settings + controls + exit

pub struct MainMenuState {

	action: Option<MenuStateAction>,

	button: Panel,
	cursor: usize,

	saves: Vec<String>,

	delete: bool,

}

impl MainMenuState {

	const GAP: f32 = 35.0;

	pub fn new(ctx: &mut Context) -> Self {
		Self {
			action: None,
			button: Panel::new(ctx),
			cursor: 0,
			saves: Vec::new(),
			delete: false,
		}
	}

	fn update_saves(&mut self, saves: &PlayerSaves) {
		self.saves = saves.saves.iter().map(|save| save.name.clone()).collect();
	}

}

impl State for MainMenuState {

	fn begin(&mut self, _ctx: &mut Context) -> Result {
		self.cursor = 0;
		self.delete = false;
		if let Some(saves) = unsafe{PLAYER_SAVES.as_ref()} {
			self.update_saves(&saves);
		}
		Ok(())
	}
	
	fn update(&mut self, ctx: &mut Context) -> Result {
		if pressed(ctx, Control::A) {
			if self.cursor == self.saves.len() {
				self.action = Some(MenuStateAction::Goto(MenuStates::CharacterCreation));
				// saves.select_new(&game::storage::player::default_name());
			} else if self.cursor == self.saves.len() + 1 {
				self.delete = !self.delete;
			} else {
				if let Some(saves) = unsafe{PLAYER_SAVES.as_mut()} {
					if self.delete {
						if saves.delete(self.cursor) {
							self.cursor -= 1;
							self.update_saves(&saves);
						};
					} else {
						saves.select(self.cursor);
						self.action = Some(MenuStateAction::StartGame);
					}					
				} else {
					warn!("Could not get player save data!");
				}
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

		for (index, save) in self.saves.iter().enumerate() {
			let y = 5.0 + index as f32 * Self::GAP;
			self.button.draw(ctx, 20.0, y, 206.0, 30.0);
			draw_text_left(ctx, &1, save, &TextColor::Black, 31.0, y + 5.0);
		}

		let saves_len = self.saves.len() as f32;

		{
			let y = 5.0 + saves_len * Self::GAP;
			self.button.draw(ctx, 20.0, y, 206.0, 30.0);
			draw_text_left(ctx, &1, "New Game", &TextColor::Black, 31.0, y + 5.0);
		}

		{
			let y = 5.0 + (saves_len + 1.0) * Self::GAP;
			self.button.draw(ctx, 20.0, y, 206.0, 30.0);
			draw_text_left(ctx, &1, if self.delete { "Play" } else { "Delete" }, &TextColor::Black, 31.0, y + 5.0);
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