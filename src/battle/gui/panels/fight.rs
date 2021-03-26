use firecore_pokedex::pokemon::battle::BattlePokemon;
use firecore_util::Reset;
use firecore_util::text::TextColor;
use macroquad::prelude::Vec2;
use firecore_util::Entity;
use smallvec::SmallVec;
use crate::battle::Battle;
use crate::battle::battle_party::BattleMoveStatus;
use crate::battle::gui::battle_text::BattleText;
use crate::util::graphics::Texture;

use firecore_input::{self as input, Control};

use crate::util::graphics::draw;
use crate::util::graphics::texture::byte_texture;

use super::moves::MovePanel;
pub struct FightPanel {

    alive: bool,

    pos: Vec2,
    panel: Vec2,

    background: Texture,
    move_panel: MovePanel,

    cursor: usize,

    move_names: SmallVec<[String; 4]>,

    pub spawn_battle_panel: bool,

}

impl FightPanel {

    pub fn new(panel: Vec2) -> FightPanel {

        FightPanel {

            alive: false,

            pos: Vec2::default(),

            background: byte_texture(include_bytes!("../../../../build/assets/gui/battle/move_panel.png")),
            move_panel: MovePanel::new(panel),
            panel,

            cursor: 0,

            move_names: SmallVec::new(),

            spawn_battle_panel: false,

        }

    }

    pub fn update_names(&mut self, instance: &BattlePokemon) {
        self.move_names.clear();
        for (index, move_instance) in instance.moves.iter().enumerate() {
            if index == 4 {
                break;
            }
            self.move_names.push(move_instance.pokemon_move.name.to_ascii_uppercase());
        }
    }

    pub fn update_move(&mut self, pokemon: &BattlePokemon) {
        if let Some(pmove) = pokemon.moves.get(self.cursor) {
            self.move_panel.update_with_move(pmove);
        }        
    }

    pub fn render(&self) {
        if self.alive {

            draw(self.background, self.pos.x + self.panel.x, self.pos.y + self.panel.y);
            self.move_panel.render();
        
            for (index, string) in self.move_names.iter().enumerate() {
                let mut x_offset = 0.0;
                let mut y_offset = 0.0;
                if index % 2 == 1 {
                    x_offset = 72.0;
                }
                if index / 2 == 1 {
                    y_offset = 17.0;
                }
                crate::util::graphics::draw_text_left(0, string, TextColor::Black, self.panel.x + self.pos.x + 16.0 + x_offset, self.panel.y + self.pos.y + 8.0 + y_offset);
                if index == self.cursor {
                    crate::util::graphics::draw_cursor(self.panel.x + self.pos.x + 10.0 + x_offset, self.panel.y + self.pos.y + 10.0 + y_offset);
                }
            }
        }        
    }

    pub fn input(&mut self, battle: &mut Battle, text: &mut BattleText) {

        if !self.move_panel.has_move {
            self.update_move(battle.player.active());
            self.move_panel.has_move = true;
        }

        if input::pressed(Control::B) {
            self.spawn_battle_panel = true;
        }

        if input::pressed(input::Control::A) {

            // Despawn the panel, set the text for the battle text, and spawn the battle text.

            self.despawn();        

            if self.cursor < battle.player.active().moves.len() {
                battle.player.next_move = BattleMoveStatus::new(battle.player.active_mut().moves[self.cursor].use_move());
            }

            let index = macroquad::rand::gen_range(0, battle.opponent.active().moves.len());
		    battle.opponent.next_move = BattleMoveStatus::new(battle.opponent.active_mut().moves[index].use_move());
            
            text.reset_text();

            let player = battle.player.active();
            let opponent = battle.opponent.active();

            text.add_moves(player.name(), &battle.player.next_move.pokemon_move.name);
            text.add_moves(opponent.name(), &battle.opponent.next_move.pokemon_move.name);
        
            if player.base.speed < opponent.base.speed {
                if let Some(messages) = text.text.messages.as_mut() {
                    messages.swap(0, 1);
                }
            }

            text.text.spawn();
            
        }

        let mut update_cursor = false;

        if input::pressed(Control::Up) {
            if self.cursor >= 2 {
                self.cursor -= 2;
                update_cursor = true;
            }            
        } else if input::pressed(Control::Down) {
            if self.cursor <= 2 {
                self.cursor += 2;
                update_cursor = true;
            } 
        } else if input::pressed(Control::Left) {
            if self.cursor > 0 {
                self.cursor -= 1;
                update_cursor = true;
            }
        } else if input::pressed(Control::Right) {
            if self.cursor < 3 {
                self.cursor += 1;
                update_cursor = true;
            }
        }

        if update_cursor {
            if self.cursor >= self.move_names.len() {
                self.cursor = self.move_names.len() - 1;
            }
            self.update_move(battle.player.active());
        }

    }

}

impl Entity for FightPanel {

    fn spawn(&mut self) {
        self.alive = true;
        self.reset();
    }

    fn despawn(&mut self) {
        self.alive = false;
    }

    fn is_alive(& self) -> bool {
        self.alive
    }

}

impl Reset for FightPanel {
    fn reset(&mut self) {
        self.cursor = 0;
        self.move_panel.has_move = false;
    }    
}

pub enum FightPanelNext {

    BattlePanel,
    BattleMove,

}