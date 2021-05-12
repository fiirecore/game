use game::{
    util::{
        Reset, 
        Completable,
    },
    text::MessagePage,
    gui::DynamicText,
    graphics::draw_o_bottom,
    macroquad::prelude::Texture2D,
};

use crate::{
    Battle,
    ui::transitions::{
        BattleIntroduction,
        introductions::basic::BasicBattleIntroduction,
    }
};

#[derive(Default)]
pub struct TrainerBattleIntroduction {

    introduction: BasicBattleIntroduction,

    texture: Option<Texture2D>,
    offset: f32,
    leaving: bool,

}

impl TrainerBattleIntroduction {
    const FINAL_TRAINER_OFFSET: f32 = 126.0;
}

impl BattleIntroduction for TrainerBattleIntroduction {

    fn spawn(&mut self, battle: &Battle, text: &mut DynamicText) {
        text.clear();

        if let Some(trainer) = battle.data.trainer.as_ref() {
            self.texture = Some(trainer.texture);

            let name = format!("{} {}", trainer.npc_type, trainer.name);

            text.push(MessagePage::new(
                vec![
                    name.clone(), 
                    String::from("would like to battle!")
                ], 
                None
            ));

            text.push(MessagePage::new(
                vec![
                    name + " sent", 
                    format!("out {}", BasicBattleIntroduction::concatenate(&battle.opponent.active))
                ],
                Some(0.5),
            ));
        } else {
            text.push(MessagePage::new(
                vec![String::from("No trainer data found!")],
                None,
            ));
        }

        self.introduction.common_setup(text, &battle.player.active);
        
    }

    fn update(&mut self, delta: f32, battle: &mut Battle, text: &mut DynamicText) {
        self.introduction.update(delta, battle, text);
        if text.can_continue() {
            if text.current() == text.len() - 2 {
                self.leaving = true;
            }           
        }
        if self.leaving && self.offset < Self::FINAL_TRAINER_OFFSET {
            self.offset += 300.0 * delta;
        }
    }

    fn render(&self, battle: &Battle) {
        if self.offset < Self::FINAL_TRAINER_OFFSET {
            draw_o_bottom(self.texture, 144.0 + self.offset, 74.0);
        } else {
            self.introduction.render_opponent(battle);
        }
        self.introduction.render_player(battle);  
    }
}

impl Completable for TrainerBattleIntroduction {
    fn is_finished(&self) -> bool {
        self.introduction.is_finished()
    }
}

impl Reset for TrainerBattleIntroduction {
    fn reset(&mut self) {
        self.introduction.reset();
        self.offset = 0.0;
        self.leaving = false;
    }
}