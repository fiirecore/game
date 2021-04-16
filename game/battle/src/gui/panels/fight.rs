use game::{
    util::{Entity, Reset},
    pokedex::pokemon::instance::PokemonInstance,
    input::{pressed, Control},
    macroquad::prelude::Vec2
};

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

            if let Some(player_move) = BattleMoveStatus::new(battle.player.active_mut().moves[self.move_panel.cursor].use_move()) {

                self.despawn();        

                if self.move_panel.cursor < battle.player.active().moves.len() {
                    battle.player.next_move = Some(player_move);
                }
    
                let index = crate::BATTLE_RANDOM.gen_range(0..battle.opponent.active().moves.len() as u32) as usize;
                battle.opponent.next_move = BattleMoveStatus::new(battle.opponent.active_mut().moves[index].use_move());
                
                text.reset_text();
    
                let player = battle.player.active();
                let opponent = battle.opponent.active();
    
                text.add_moves(player.name(), &battle.player.next_move.as_ref().unwrap().pokemon_move.name);
                text.add_moves(opponent.name(), &battle.opponent.next_move.as_ref().unwrap().pokemon_move.name);
            
                if player.base.speed < opponent.base.speed {
                    if let Some(messages) = text.text.messages.as_mut() {
                        messages.swap(0, 1);
                    }
                }
    
                text.text.spawn();

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