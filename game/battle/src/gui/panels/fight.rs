use game::{
    util::{Entity, Reset},
    pokedex::pokemon::instance::PokemonInstance,
    input::{pressed, Control},
    macroquad::prelude::Vec2
};

use crate::pokemon::BattleMoveType;
use crate::{
    Battle,
    pokemon::BattleMoveStatus,
    gui::{
        text::BattleText,
        panels::{
            moves::MovePanel,
            move_info::MoveInfoPanel,
        },
    }
};

pub struct FightPanel {

    move_panel: MovePanel,
    move_info: MoveInfoPanel,

    pub spawn_battle_panel: bool,

}

impl FightPanel {

    pub fn new(panel: Vec2) -> FightPanel {

        FightPanel {

            move_panel: MovePanel::new(panel),
            move_info: MoveInfoPanel::new(panel),

            spawn_battle_panel: false,

        }

    }

    pub fn update_gui(&mut self, instance: &PokemonInstance) {
        self.move_panel.update_names(instance);
        self.update_move(instance);
    }

    pub fn update_move(&mut self, pokemon: &PokemonInstance) {
        if let Some(pmove) = pokemon.moves.get(self.move_panel.cursor) {
            self.move_info.update_with_move(pmove);
        }        
    }

    pub fn render(&self) {
        if self.move_panel.is_alive() {
            self.move_panel.render();
            self.move_info.render();
        }        
    }

    pub fn input(&mut self, battle: &mut Battle, text: &mut BattleText) {

        if self.move_panel.input() {
            self.update_move(battle.player.active());
        }

        if pressed(Control::B) {
            self.spawn_battle_panel = true;
        }

        if pressed(Control::A) {

            // Despawn the panel, set the text for the battle text, and spawn the battle text.

            if let Some(pokemon_move) = battle.player.active_mut().moves.get_mut(self.move_panel.cursor).map(|instance| instance.use_move()).flatten() {
                self.despawn();

                battle.player.next_move = Some(BattleMoveStatus::new(BattleMoveType::Move(pokemon_move)));

                text.run(battle);

            }
            
        }

    }

}

impl Entity for FightPanel {

    fn spawn(&mut self) {
        self.move_panel.spawn();
        self.reset();
    }

    fn despawn(&mut self) {
        self.move_panel.despawn();
    }

    fn is_alive(& self) -> bool {
        self.move_panel.is_alive()
    }

}

impl Reset for FightPanel {
    fn reset(&mut self) {
        self.move_panel.reset();
    }    
}

pub enum FightPanelNext {

    BattlePanel,
    BattleMove,

}