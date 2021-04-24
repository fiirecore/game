use game::macroquad::prelude::warn;
use game::pokedex::item::ItemRef;
use game::pokedex::moves::MoveRef;
use game::pokedex::pokemon::types::effective::Effective;
use game::{
    util::text::{Message, TextColor},
    pokedex::pokemon::instance::PokemonInstance,
    macroquad::prelude::Vec2,
};

use crate::Battle;
use crate::pokemon::BattleMoveType;
use crate::pokemon::BattleParty;
use game::gui::text::DynamicText;
use super::pokemon::PokemonGui;

pub struct BattleText {

    pub text: DynamicText,

    pub player_index: Option<usize>,
    pub opponent_index: Option<usize>,
    pub post_index: Option<usize>,
    pub faint_index: Option<usize>,

}

impl BattleText {

    pub fn new() -> Self {
        Self {
            text: DynamicText::with_size(5, Vec2::new(11.0, 11.0), Vec2::new(0.0, 113.0)),

            player_index: None,
            opponent_index: None,
            post_index: None,
            faint_index: None,
        }
    }

    pub fn reset_text(&mut self) {
        if let Some(messages) = self.text.messages.as_mut() {
            messages.clear();
            self.player_index = None;
            self.opponent_index = None;
            self.faint_index = None;
        }
    }

    pub fn run(&mut self, battle: &mut Battle) {
        battle.generate_opponent_move();

        self.reset_text();

        self.add_move_status(battle);

        game::util::Entity::spawn(&mut self.text);
    }

    pub fn add_move_status(&mut self, battle: &Battle) {
        if self.text.messages.is_some() {
            let player_first = battle.player_first();
            if player_first {
                self.add_player(battle);
            }

            if let Some(action) = battle.opponent.next_move.as_ref().map(|status| status.action) {
                self.opponent_index = Some(self.text.messages.as_ref().unwrap().len());
                self.add_user(&battle.opponent, &battle.player, action, false);
            }

            if !player_first {
                self.add_player(battle);
            }

            if let Some(messages) = self.text.messages.as_mut() {

                self.post_index = Some(messages.len());

                if !battle.opponent.active().persistent.is_empty() {
                    messages.push(Message::new(
                        vec![format!("{} was hurt by Leech Seed!", battle.opponent.active().name())], 
                        TextColor::White,
                        Some(0.5),
                    ))

                }

            }

        }
        
    }

    fn add_player(&mut self, battle: &Battle) {
        if let Some(action) = battle.player.next_move.as_ref().map(|status| status.action) {
            self.player_index = Some(self.text.messages.as_ref().unwrap().len());
            self.add_user(&battle.player, &battle.opponent, action, true);
        } else {
            warn!("Could not add player text, no move action was found.");
        }
    }

    fn add_user(&mut self, user: &BattleParty, opponent: &BattleParty, action: BattleMoveType, player: bool) {
        match action {
            BattleMoveType::Move(pokemon_move) => self.add_move(user.active(), pokemon_move, opponent.active()),
            BattleMoveType::UseItem(item) => self.add_item(user.active(), item),
            BattleMoveType::Switch(index) => self.add_switch(user.active(), &user.pokemon[index].pokemon, player),
        }
    }

    fn add_move(&mut self, user: &PokemonInstance, pokemon_move: MoveRef, opponent: &PokemonInstance)  {
        if let Some(messages) = self.text.messages.as_mut() {
            messages.push(
                Message::new(
                    vec![user.name() + " used " + &pokemon_move.name + "!"],
                    TextColor::White,
                    Some(0.5),
                )
            );

            let effective = opponent.move_effective(pokemon_move);

            if effective != Effective::Effective {
                messages.push(Message::new(vec![format!("It was {}{}", effective, if effective == Effective::SuperEffective { "!" } else { "..." })], TextColor::White, Some(0.5)));
            }
            
        } else {
            warn!("Could not add move messages because text has not been initialized!");
        }
    }

    fn add_item(&mut self, user: &PokemonInstance, item: ItemRef) {
        if let Some(messages) = self.text.messages.as_mut() {
            messages.push(
                Message::new(
                    vec![format!("A {} was used on {}", item.name, user.name())], 
                    TextColor::White, 
                    Some(0.5)
                )
            );
        } else {
            warn!("Could not add item use messages because text has not been initialized!");
        }
    }

    fn add_switch(&mut self, leaving: &PokemonInstance, coming: &PokemonInstance, player: bool) {
        if let Some(messages) = self.text.messages.as_mut() {
            if let Some(index) = if player { &mut self.player_index } else { &mut self.opponent_index } {
                *index += 1;
            }
            messages.push(
                Message::new(
                    vec![format!("Come back, {}!", leaving.name())], 
                    TextColor::White, 
                    Some(0.5),
                )
            );
            messages.push(
                Message::new(
                    vec![format!("Go, {}!", coming.name())], 
                    TextColor::White, 
                    Some(0.5),
                )
            );
        } else {
            warn!("Could not add party switch messages because text has not been initialized!");
        }
    }

    pub fn add_faint_text(&mut self, name: String) {
        if let Some(messages) = self.text.messages.as_mut() {
            self.faint_index = Some(messages.len());
            messages.push(
                Message::new(
                    vec![name + " fainted!"],
                    TextColor::White,
                    Some(1.0), 
                )            
            );
        }
    }

    pub fn player_level_up(&mut self, name: String, exp: u32, level: Option<u8>) {
        if let Some(messages) = self.text.messages.as_mut() {
            messages.push(
                Message::new(
                    vec![
                        format!("{} gained", name),
                        format!("{} EXP. points!", exp),
                    ],
                    TextColor::White,
                    None,
                )
            );
            if let Some(level) = level {
                messages.push(
                    Message::new(
                        vec![
                            name + " grew to",
                            format!("LV. {}!", level),
                        ],
                        TextColor::White,
                        Some(0.5),
                    )
                );                
            }
        }        
    }

    pub fn on_move(&mut self, other_pokemon: &PokemonInstance, gui: &mut impl PokemonGui) {

        gui.update_gui(other_pokemon, false);

        if other_pokemon.is_faint() {
            let next = self.text.current_message() + 1;
            if let Some(messages) = self.text.messages.as_mut() {
                if next < messages.len() {
                    messages.remove(next);                
                }
            }            
            self.add_faint_text(other_pokemon.name());
        }

    }

    pub fn perform_player(&self, battle: &Battle) -> bool {

        self.text.can_continue && 
        battle.player.next_move_queued() && 
        !battle.player.active().is_faint() && 
        self.player_index.map(|index| index == self.text.current_message()).unwrap_or_default()
    }

    pub fn perform_opponent(&self, battle: &Battle) -> bool {
        self.text.can_continue && 
        battle.opponent.next_move_queued() && 
        !battle.opponent.active().is_faint() && 
        self.opponent_index.map(|index| index == self.text.current_message()).unwrap_or_default()
    }

    pub fn perform_post(&self, battle: &Battle) -> bool {
        battle.post_run &&
        self.text.can_continue &&
        !battle.player.next_move_queued() &&
        !battle.opponent.next_move_queued() &&
        self.post_index.map(|index| index == self.text.current_message()).unwrap_or_default()
    }

}