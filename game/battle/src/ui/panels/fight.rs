use game::{
    pokedex::pokemon::instance::PokemonInstance,
    input::{pressed, Control},
};

use crate::ui::panels::{
    moves::MovePanel,
    move_info::MoveInfoPanel,
    target::TargetPanel,
};

pub struct FightPanel {

    target_active: bool,

    pub moves: MovePanel,
    pub targets: TargetPanel,
    info: MoveInfoPanel,

}

impl FightPanel {

    pub fn new() -> FightPanel {

        FightPanel {

            target_active: false,

            moves: MovePanel::new(),
            targets: TargetPanel::new(),
            info: MoveInfoPanel::new(),

        }

    }

    pub fn setup(&mut self, instance: &PokemonInstance, targets: &crate::pokemon::ActivePokemonArray) {
        self.moves.update_names(instance);
        self.targets.update_names(targets);
        self.update_move(instance);
    }

    pub fn update_move(&mut self, pokemon: &PokemonInstance) {
        if let Some(pmove) = pokemon.moves.get(self.moves.cursor) {
            self.info.update_move(pmove);
        }
    }

    pub fn render(&self) {
        if !self.target_active {
            self.moves.render();
            self.info.render();  
        } else {
            self.targets.render();
        }
    }

    pub fn input(&mut self, pokemon: &PokemonInstance) -> bool {
        if self.target_active {
            self.targets.input();
            if pressed(Control::B) {
                self.target_active = false;
            }
            pressed(Control::A)
        } else {
            if self.moves.input() {
                self.update_move(pokemon);
            }
            if pressed(Control::A) {
                if self.targets.names.len() == 1 {
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

impl game::util::Reset for FightPanel {
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