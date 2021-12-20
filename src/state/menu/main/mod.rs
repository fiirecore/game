mod gui;

use std::borrow::Cow;

use crate::{
    engine::{
        graphics::{draw_rectangle, draw_rectangle_lines, draw_text_left, Color, DrawParams},
        input::{controls::{pressed, Control}, mouse},
        math::Vec2,
        utils::{HEIGHT, WIDTH},
        Context,
        text::MessagePage,
    },
    saves::SavedPlayer,
};

use firecore_world::events::Sender;

use gui::{Button, ButtonBase};

use super::{MenuActions, MenuStates};

// have normal main menu + video settings + controls + exit

pub struct MainMenuState {
    cursor: usize,

    save_buttons: Vec<Button>,

    delete: bool,

    last_mouse_pos: Vec2,

    new_game: ButtonBase,
    delete_button: ButtonBase,

    sender: Sender<MenuActions>,
}

impl MainMenuState {
    const GAP: f32 = 35.0;

    pub(crate)fn new(sender: Sender<MenuActions>) -> Self {
        Self {
            cursor: Default::default(),
            save_buttons: Vec::new(),
            delete: false,
            last_mouse_pos: Default::default(),
            new_game: ButtonBase::new(Vec2::new(206.0, 30.0), Cow::Borrowed("New Game")),
            delete_button: ButtonBase::new(Vec2::new(206.0, 30.0), Cow::Borrowed("Play/Delete")),
            sender,
        }
    }

    fn update_saves(saves: &[SavedPlayer], list: &mut Vec<Button>) {
        *list = saves
            .iter()
            .enumerate()
            .map(|(index, save)| {
                Button::new(
                    Vec2::new(20.0, 5.0 + index as f32 * Self::GAP),
                    Vec2::new(206.0, 30.0),
                    Cow::Borrowed(unsafe { &*(save.character.name.as_str() as *const str) }),
                )
            })
            .collect();
    }
}

impl MainMenuState {
    pub fn start(&mut self, saves: &[SavedPlayer]) {
        self.cursor = Default::default();
        self.delete = false;
        Self::update_saves(saves, &mut self.save_buttons);

        // Ok(())
    }

    pub fn update(&mut self, ctx: &mut Context, saves: &mut Vec<SavedPlayer>) {

        if pressed(ctx, Control::Start) {
            let index = saves.len();
            saves.push(crate::saves::Player::new(
                format!(
                    "Player {}",
                    crate::engine::utils::seed()
                ),
            ));
            self.sender.send(MenuActions::StartGame(index));
        }

        let mouse_pos = mouse::position(ctx);

        let last = if self.last_mouse_pos != mouse_pos {
            self.last_mouse_pos = mouse_pos;
            true
        } else {
            false
        };

        for (index, button) in self.save_buttons.iter_mut().enumerate() {
            let (click, mouse) = button.update(ctx, last.then(|| mouse_pos), self.cursor == index);
            if mouse {
                self.cursor = index;
            }
            if click {
                if self.delete {
                    if saves.len() < index {
                        saves.remove(index);
                        // if index >= self.cursor {
                        // 	self.cursor -= 1;
                        // }
                        Self::update_saves(saves, &mut self.save_buttons);
                        break;
                    };
                } else {
                    self.sender.send(MenuActions::StartGame(index));
                }
            }
        }

        let new_game_pos = saves.len();

        {
            let (click, mouse) = self.new_game.update(
                ctx,
                &Vec2::new(20.0, 5.0 + new_game_pos as f32 * Self::GAP),
                last.then(|| mouse_pos),
                self.cursor == new_game_pos,
            );

            if mouse {
                self.cursor = new_game_pos;
            }

            if click {
                saves.push(crate::saves::Player::new(
                        format!(
                            "Player {}",
                            crate::engine::utils::seed()
                        ),
                    ));
            }
        }

        let delete_pos = new_game_pos + 1;

        {
            let (click, mouse) = self.delete_button.update(
                ctx,
                &Vec2::new(20.0, 5.0 + delete_pos as f32 * Self::GAP),
                last.then(|| mouse_pos),
                self.cursor == delete_pos,
            );

            if mouse {
                self.cursor = delete_pos;
            }

            if click {
                self.delete = !self.delete;
            }
        }

        if pressed(ctx, Control::B) {
            self.sender.send(MenuActions::Goto(MenuStates::Title));
        }

        if pressed(ctx, Control::Up) && self.cursor > 0 {
            self.cursor -= 1;
        }

        if pressed(ctx, Control::Down) && self.cursor <= saves.len() {
            self.cursor += 1;
        }

        // Ok(())
    }

    pub fn draw(&mut self, ctx: &mut Context, saves: &[SavedPlayer]) {
        draw_rectangle(ctx, 0.0, 0.0, WIDTH, HEIGHT, Color::rgb(0.00, 0.32, 0.67));

        for save in self.save_buttons.iter() {
            save.draw(ctx);
            // self.button.draw(ctx, 20.0, y, 206.0, 30.0);
            // draw_text_left(ctx, &1, save, &Message::Black, 31.0, y + 5.0);
        }

        let saves_len = saves.len() as f32;

        {
            let y = 5.0 + saves_len * Self::GAP;
            self.new_game.draw(ctx, Vec2::new(20.0, y));
            // 	draw_text_left(ctx, &1, "New Game", &Message::Black, 31.0, y + 5.0);
        }

        {
            let y = 5.0 + (saves_len + 1.0) * Self::GAP;
            self.delete_button.draw(ctx, Vec2::new(20.0, y));
            // 	draw_text_left(ctx, &1, &Message::Black, 31.0, y + 5.0);
        }

        draw_rectangle_lines(
            ctx,
            20.0,
            5.0 + self.cursor as f32 * Self::GAP,
            206.0,
            30.0,
            2.0,
            Color::rgb(1.0, 0.0, 0.0),
        );

        draw_text_left(
            ctx,
            &1,
            if self.delete {
                "Delete Mode: ON"
            } else {
                "Delete Mode: OFF"
            },
            5.0,
            145.0,
            DrawParams::color(MessagePage::BLACK),
        );

        // Ok(())
    }
}
