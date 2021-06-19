use crate::{
    util::{Reset, Completable},
    pokedex::trainer::TrainerData,
    storage::data as save,
    text::MessagePage,
    gui::TextDisplay, 
    graphics::draw_o_bottom, 
    tetra::{
        Context,
        graphics::Texture,
    },
};

use battle::{
    data::BattleType,
    pokemon::view::{
        BattlePartyKnown, 
        BattlePartyUnknown,
    },
};

use crate::battle_cli::ui::view::{
    ActivePokemonParty,
    ActiveRenderer,
};

use super::{BattleIntroduction, basic::BasicBattleIntroduction};

pub struct TrainerBattleIntroduction {

    introduction: BasicBattleIntroduction,

    texture: Option<Texture>,
    offset: f32,
    leaving: bool,

}

impl TrainerBattleIntroduction {

    const FINAL_TRAINER_OFFSET: f32 = 126.0;

    pub fn new(ctx: &mut Context) -> Self {
        Self {
            introduction: BasicBattleIntroduction::new(ctx),
            texture: None,
            offset: 0.0,
            leaving: false,
        }
    }

}

impl BattleIntroduction for TrainerBattleIntroduction {

    fn spawn(&mut self, _battle_type: BattleType, player: &BattlePartyKnown, opponent: &BattlePartyUnknown, text: &mut TextDisplay) {
        text.clear();

        if let Some(trainer) = &opponent.trainer {
            self.texture = Some(pokedex::texture::trainer::trainer_texture(&trainer.npc_type).clone());

            let name = format!("{} {}", trainer.prefix, trainer.name);

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
                    format!("out {}", BasicBattleIntroduction::concatenate(opponent))
                ],
                Some(0.5),
            ));
        } else {
            text.push(MessagePage::new(
                vec![String::from("No trainer data found!")],
                None,
            ));
        }

        text.process_messages(save());

        self.introduction.common_setup(text, player);
        
    }

    fn update(&mut self, ctx: &Context, delta: f32, player: &mut ActivePokemonParty<BattlePartyKnown>, opponent: &mut ActivePokemonParty<BattlePartyUnknown>, text: &mut TextDisplay) {
        self.introduction.update(ctx, delta, player, opponent, text);
        if text.can_continue() && text.current() == text.len() - 2 {
            self.leaving = true;          
        }
        if self.leaving && self.offset < Self::FINAL_TRAINER_OFFSET {
            self.offset += 300.0 * delta;
        }
    }

    fn draw(&self, ctx: &mut Context, player: &ActiveRenderer, opponent: &ActiveRenderer) {
        if self.offset < Self::FINAL_TRAINER_OFFSET {
            draw_o_bottom(ctx, self.texture.as_ref(), 144.0 + self.offset, 74.0);
        } else {
            self.introduction.draw_opponent(ctx, opponent);
        }
        self.introduction.draw_player(ctx, player);  
    }
}

impl Completable for TrainerBattleIntroduction {
    fn finished(&self) -> bool {
        self.introduction.finished()
    }
}

impl Reset for TrainerBattleIntroduction {
    fn reset(&mut self) {
        self.introduction.reset();
        self.offset = 0.0;
        self.leaving = false;
    }
}