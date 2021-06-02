use crate::{battle::pokemon::BattlePartyView, input::{pressed, Control}, pokedex::pokemon::instance::PokemonInstance, tetra::Context, util::Reset};

use crate::battle::{
    pokemon::ActivePokemonArray,
    ui::panels::{
        moves::MovePanel,
        move_info::MoveInfoPanel,
        target::TargetPanel,
    },
};

pub struct FightPanel {

    target_active: bool,

    pub moves: MovePanel,
    // #[deprecated(note = "fix")]
    pub targets: TargetPanel,
    info: MoveInfoPanel,

}

impl FightPanel {

    pub fn new(ctx: &mut Context) -> FightPanel {

        FightPanel {

            target_active: false,

            moves: MovePanel::new(ctx),
            targets: TargetPanel::new(ctx),
            info: MoveInfoPanel::new(ctx),

        }

    }

    pub fn user(&mut self, instance: &PokemonInstance) {
        self.moves.update_names(instance);
        self.update_move(instance);
    }

    pub fn target(&mut self, targets: &BattlePartyView) {
        self.targets.update_names(targets);
    }

    pub fn update_move(&mut self, pokemon: &PokemonInstance) {
        if let Some(pmove) = pokemon.moves.get(self.moves.cursor) {
            self.info.update_move(pmove);
        }
    }

    pub fn draw(&self, ctx: &mut Context) {
        if !self.target_active {
            self.moves.draw(ctx);
            self.info.draw(ctx);  
        } else {
            self.targets.draw(ctx);
        }
    }

    pub fn input(&mut self, ctx: &Context, pokemon: &PokemonInstance) -> bool {
        if self.target_active {
            self.targets.input(ctx);
            if pressed(ctx, Control::B) {
                self.target_active = false;
            }
            pressed(ctx, Control::A)
        } else {
            if self.moves.input(ctx) {
                self.update_move(pokemon);
            }
            if pressed(ctx, Control::A) {
                if self.targets.names.len() <= 1 {
                    self.targets.cursor = 0;
                    true
                } else {
                    self.target_active = true;
                    false
                }
            } else {
                false
            }
        }
    }

}

impl Reset for FightPanel {
    fn reset(&mut self) {
        self.target_active = false;
        let len = self.moves.names.len();
        if self.moves.cursor >= len {
            self.moves.cursor = len.saturating_sub(1);
        }
        let len = self.targets.names.len();
        if self.targets.cursor >= len {
            self.targets.cursor = len.saturating_sub(1);
        }
    }
}