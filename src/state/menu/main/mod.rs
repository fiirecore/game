mod gui;

use std::borrow::Cow;

use crate::{
    engine::{
        graphics::{draw_rectangle, Color},
        input::controls::{pressed, Control},
        math::Vec2,
        utils::{HEIGHT, WIDTH},
        Context,
    },
    saves::Player,
};

use firecore_world::events::Sender;

use gui::Button;

use super::{MenuActions, MenuStates};

// have normal main menu + video settings + controls + exit

pub struct MainMenuState {
    /// 0 = New Game/Continue, 1 = Delete Save, 2 = Exit Game
    buttons: [Button; 3],

    cursor: usize,
    delete: bool,

    sender: Sender<MenuActions>,
}

impl MainMenuState {
    const GAP: usize = 35;

    pub(crate) fn new(sender: Sender<MenuActions>) -> Self {
        Self {
            buttons: [
                Button::new(Vec2::new(206.0, 30.0), Cow::Borrowed("Play Game")),
                Button::new(Vec2::new(206.0, 30.0), Cow::Borrowed("Delete Save")),
                Button::new(Vec2::new(206.0, 30.0), Cow::Borrowed("Exit Game")),
            ],
            cursor: Default::default(),
            delete: false,
            sender,
        }
    }
}

impl MainMenuState {
    pub fn start(&mut self) {
        self.cursor = Default::default();
        self.delete = false;

        // Ok(())
    }

    pub fn update(&mut self, ctx: &mut Context, save: &mut Option<Player>) {
        for (index, button) in self.buttons.iter_mut().enumerate() {
            if button.update(ctx, self.cursor == index) {
                match index {
                    0 => match save.is_some() {
                        true => self.sender.send(MenuActions::StartGame),
                        false => *save = Some(Player::new("Red")),
                    },
                    1 => *save = None,
                    2 => self.sender.send(MenuActions::ExitGame),
                    _ => unreachable!(),
                }
            }
        }

        if pressed(ctx, Control::B) {
            self.sender.send(MenuActions::Goto(MenuStates::Title));
        }

        if pressed(ctx, Control::Up) && self.cursor > 0 {
            self.cursor -= 1;
        }

        if pressed(ctx, Control::Down) && self.cursor < self.buttons.len() - 1 {
            self.cursor += 1;
        }

        // Ok(())
    }

    pub fn draw(&mut self, ctx: &mut Context) {
        draw_rectangle(ctx, 0.0, 0.0, WIDTH, HEIGHT, Color::rgb(0.00, 0.32, 0.67));

        for (index, save) in self.buttons.iter().enumerate() {
            save.draw(ctx, Vec2::new(20.0, (5 + index * Self::GAP) as f32));
            // self.button.draw(ctx, 20.0, y, 206.0, 30.0);
            // draw_text_left(ctx, &1, save, &Message::Black, 31.0, y + 5.0);
        }

        // let saves_len = saves.len() as f32;

        // {
        //     let y = 5.0 + saves_len * Self::GAP;
        //     self.new_game.draw(ctx, Vec2::new(20.0, y));
        //     // 	draw_text_left(ctx, &1, "New Game", &Message::Black, 31.0, y + 5.0);
        // }

        // {
        //     let y = 5.0 + (saves_len + 1.0) * Self::GAP;
        //     self.delete_button.draw(ctx, Vec2::new(20.0, y));
        //     // 	draw_text_left(ctx, &1, &Message::Black, 31.0, y + 5.0);
        // }

        // draw_rectangle_lines(
        //     ctx,
        //     20.0,
        //     5.0 + self.cursor as f32 * Self::GAP,
        //     206.0,
        //     30.0,
        //     2.0,
        //     Color::rgb(1.0, 0.0, 0.0),
        // );

        // draw_text_left(
        //     ctx,
        //     &1,
        //     if self.delete {
        //         "Delete Mode: ON"
        //     } else {
        //         "Delete Mode: OFF"
        //     },
        //     5.0,
        //     145.0,
        //     DrawParams::color(MessagePage::BLACK),
        // );

        // Ok(())
    }
}
