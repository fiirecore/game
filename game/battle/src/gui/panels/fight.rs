use game::{
    pokedex::pokemon::instance::PokemonInstance,
    input::{pressed, Control},
    macroquad::prelude::Vec2
};

use crate::gui::panels::{
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

            moves: MovePanel::new(Vec2::new(0.0, 113.0)),
            targets: TargetPanel::new(),
            info: MoveInfoPanel::new(),

        }

    }

    pub fn setup(&mut self, instance: &PokemonInstance, targets: &Box<[crate::pokemon::ActivePokemon]>) {
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
                self.target_active = true;
            }
            false
        }
    }

}

impl game::util::Reset for FightPanel {
    fn reset(&mut self) {
        self.target_active = false;
        if self.moves.names.len() != 0 {
            self.moves.cursor = self.moves.cursor.clamp(0, self.moves.names.len() - 1);
        }
        if self.targets.names.len() != 0 {
            self.targets.cursor = self.targets.cursor.clamp(0, self.targets.names.len() - 1);
        }
    }
}