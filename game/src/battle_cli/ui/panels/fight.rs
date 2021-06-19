use crate::{
    util::Reset,
    pokedex::pokemon::instance::PokemonInstance, 
    tetra::Context, 
};

use crate::battle_cli::{
    ui::panels::{
        moves::MovePanel,
        move_info::MoveInfoPanel,
    },
};

pub struct FightPanel {

    pub moves: MovePanel,
    // #[deprecated(note = "fix")]
    info: MoveInfoPanel,

}

impl FightPanel {

    pub fn new(ctx: &mut Context) -> Self {
        Self {
            moves: MovePanel::new(ctx),
            info: MoveInfoPanel::new(ctx),
        }
    }

    pub fn user(&mut self, instance: &PokemonInstance) {
        self.moves.update_names(instance);
        self.update_move(instance);
    }

    pub fn update_move(&mut self, pokemon: &PokemonInstance) {
        if let Some(pmove) = pokemon.moves.get(self.moves.cursor) {
            self.info.update_move(pmove);
        }
    }

    pub fn draw(&self, ctx: &mut Context) {
        self.moves.draw(ctx);
        self.info.draw(ctx);  
    }

    pub fn input(&mut self, ctx: &Context, pokemon: &PokemonInstance) {
        if self.moves.input(ctx) {
            self.update_move(pokemon);
        }
    }

}

impl Reset for FightPanel {
    fn reset(&mut self) {
        if self.moves.cursor >= self.moves.names.len() {
            self.moves.cursor = 0;
        }
    }
}