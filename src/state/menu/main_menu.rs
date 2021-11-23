use std::borrow::Cow;

use crate::{
    engine::{
        graphics::{draw_rectangle, draw_rectangle_lines, draw_text_left, Color, DrawParams},
        input::{pressed, Control},
        math::Vec2,
        text::TextColor,
        util::{HEIGHT, WIDTH},
        State,
    },
    game::gui::{Button, ButtonBase},
    GameContext,
};

use super::{MenuState, MenuStateAction, MenuStates};

// have normal main menu + video settings + controls + exit

pub struct MainMenuState {
    action: Option<MenuStateAction>,

    cursor: usize,

    saves: Vec<Button>,

    delete: bool,

    last_mouse_pos: Vec2,

    new_game: ButtonBase,
    delete_button: ButtonBase,

}

impl MainMenuState {
    const GAP: f32 = 35.0;

    pub fn new() -> Self {
        Self {
            action: None,
            cursor: Default::default(),
            saves: Vec::new(),
            delete: false,
            last_mouse_pos: Default::default(),
            new_game: ButtonBase::new(Vec2::new(206.0, 30.0), Cow::Borrowed("New Game")),
            delete_button: ButtonBase::new(Vec2::new(206.0, 30.0), Cow::Borrowed("Play/Delete")),
        }
    }

    fn update_saves(ctx: &mut GameContext, list: &mut Vec<Button>) {
        *list = ctx
            .saves
            .list
            .iter()
            .enumerate()
            .map(|(index, save)| {
                Button::new(
                    Vec2::new(20.0, 5.0 + index as f32 * Self::GAP),
                    Vec2::new(206.0, 30.0),
                    Cow::Borrowed(unsafe { &*(save.name.as_str() as *const str) }),
                )
            })
            .collect();
    }
}

impl<'d> State<GameContext> for MainMenuState {
    fn start(&mut self, ctx: &mut GameContext) {
        self.cursor = Default::default();
        self.delete = false;
        Self::update_saves(ctx, &mut self.saves);

        // Ok(())
    }

    fn update(&mut self, ctx: &mut GameContext, _: f32) {
        let mouse_pos = crate::engine::inner::prelude::mouse_position().into();

        let last = if self.last_mouse_pos != mouse_pos {
            self.last_mouse_pos = mouse_pos;
            true
        } else {
            false
        };

        for (index, button) in self.saves.iter_mut().enumerate() {
            let (click, mouse) =
                button.update(&ctx.engine, last.then(|| mouse_pos), self.cursor == index);
            if mouse {
                self.cursor = index;
            }
            if click {
                if self.delete {
                    if ctx.saves.delete(index) {
                        // if index >= self.cursor {
                        // 	self.cursor -= 1;
                        // }
                        Self::update_saves(ctx, &mut self.saves);
                        break;
                    };
                } else {
                    ctx.saves.select(
                        index,
                        &mut ctx.random,
                        crate::pokedex(),
                        crate::movedex(),
                        crate::itemdex(),
                    );
                    self.action = Some(MenuStateAction::StartGame);
                }
            }
        }

        let new_game_pos = self.saves.len();

        {
            let (click, mouse) = self.new_game.update(
                &ctx.engine,
                &Vec2::new(20.0, 5.0 + new_game_pos as f32 * Self::GAP),
                last.then(|| mouse_pos),
                self.cursor == new_game_pos,
            );

            if mouse {
                self.cursor = new_game_pos;
            }

            if click {
                self.action = Some(MenuStateAction::Goto(MenuStates::CharacterCreation));
            }
        }

        let delete_pos = new_game_pos + 1;

        {
            let (click, mouse) = self.delete_button.update(
                &ctx.engine,
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

        if pressed(&ctx.engine, Control::B) {
            self.action = Some(MenuStateAction::Goto(MenuStates::Title));
        }

        if pressed(&ctx.engine, Control::Up) && self.cursor > 0 {
            self.cursor -= 1;
        }

        if pressed(&ctx.engine, Control::Down) && self.cursor <= self.saves.len() {
            self.cursor += 1;
        }

        // Ok(())
    }

    fn draw(&mut self, ctx: &mut GameContext) {
        draw_rectangle(
            &mut ctx.engine,
            0.0,
            0.0,
            WIDTH,
            HEIGHT,
            Color::rgb(0.00, 0.32, 0.67),
        );

        for save in self.saves.iter() {
            save.draw(&mut ctx.engine);
            // self.button.draw(ctx, 20.0, y, 206.0, 30.0);
            // draw_text_left(ctx, &1, save, &TextColor::Black, 31.0, y + 5.0);
        }

        let saves_len = self.saves.len() as f32;

        {
            let y = 5.0 + saves_len * Self::GAP;
            self.new_game.draw(&mut ctx.engine, Vec2::new(20.0, y));
            // 	draw_text_left(ctx, &1, "New Game", &TextColor::Black, 31.0, y + 5.0);
        }

        {
            let y = 5.0 + (saves_len + 1.0) * Self::GAP;
            self.delete_button.draw(&mut ctx.engine, Vec2::new(20.0, y));
            // 	draw_text_left(ctx, &1, &TextColor::Black, 31.0, y + 5.0);
        }

        draw_rectangle_lines(
            &mut ctx.engine,
            20.0,
            5.0 + self.cursor as f32 * Self::GAP,
            206.0,
            30.0,
            2.0,
            Color::rgb(1.0, 0.0, 0.0),
        );

        draw_text_left(
            &mut ctx.engine,
            &1,
            if self.delete {
                "Delete Mode: ON"
            } else {
                "Delete Mode: OFF"
            },
            
            5.0,
            145.0,
            DrawParams::color(TextColor::Black.into()),
        );

        // Ok(())
    }
}

impl<'d> MenuState<'d> for MainMenuState {
    fn next(&mut self) -> &mut Option<MenuStateAction> {
        &mut self.action
    }
}
