use game::macroquad::prelude::warn;
use game::pokedex::item::ItemRef;
use game::pokedex::moves::MoveCategory;
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

    pub player: Option<Text>,
    pub opponent: Option<Text>,
    pub post: Option<Text>,
    pub faint: Option<Text>,

}

pub struct Text {
    pub pos: usize,
    pub len: usize,
    pub active: usize,
}

impl Text {

    const fn new(pos: usize, len: usize) -> Self {
        Self {
            pos,
            len,
            active: 0,
        }
    }

}

impl BattleText {

    pub fn new() -> Self {
        Self {
            text: DynamicText::with_size(5, Vec2::new(11.0, 11.0), Vec2::new(0.0, 113.0)),

            player: None,
            opponent: None,
            post: None,
            faint: None,
        }
    }

    pub fn reset_text(&mut self) {
        if let Some(messages) = self.text.messages.as_mut() {
            messages.clear();
            self.player = None;
            self.opponent = None;
            self.post = None;
            self.faint = None;
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
                self.opponent = Some(Text::new(self.text.messages.as_ref().unwrap().len(), self.add_user(&battle.opponent, &battle.player, action, false)));
            }

            if !player_first {
                self.add_player(battle);
            }

            if let Some(messages) = self.text.messages.as_mut() {

                let pos = messages.len();
                let mut len = 0;

                if !battle.opponent.active().persistent.is_empty() {
                    messages.push(Message::new(
                        vec![format!("{} was hurt by Leech Seed!", battle.opponent.active().name())], 
                        TextColor::White,
                        Some(0.5),
                    ));
                    len += 1;
                }

                self.post = Some(Text::new(pos, len));

            }

        }
        
    }

    fn add_player(&mut self, battle: &Battle) {
        if let Some(action) = battle.player.next_move.as_ref().map(|status| status.action) {
            self.player = Some(Text::new(self.text.messages.as_ref().unwrap().len(), self.add_user(&battle.player, &battle.opponent, action, true)));
        } else {
            warn!("Could not add player text, no move action was found.");
        }
    }

    fn add_user(&mut self, user: &BattleParty, opponent: &BattleParty, action: BattleMoveType, player: bool) -> usize {
        match action {
            BattleMoveType::Move(pokemon_move) => self.add_move(user.active(), pokemon_move, opponent),
            BattleMoveType::UseItem(item) => self.add_item(user.active(), item),
            BattleMoveType::Switch(index) => self.add_switch(user.active(), &user.pokemon[index].pokemon, player),
        }
    }

    fn add_move(&mut self, user: &PokemonInstance, pokemon_move: MoveRef, opponent: &BattleParty) -> usize {
        let mut len = 0;
        if let Some(messages) = self.text.messages.as_mut() {
            len += 1;
            messages.push(
                Message::new(
                    vec![user.name() + " used " + &pokemon_move.name + "!"],
                    TextColor::White,
                    Some(0.5),
                )
            );

            let effective = if let Some(next_move) = opponent.next_move.as_ref() {
                if let BattleMoveType::Switch(index) = next_move.action {
                    &opponent.pokemon[index].pokemon
                } else {
                    opponent.active()
                }
            } else {
                opponent.active()
            }.move_effective(pokemon_move);


            if effective != Effective::Effective && pokemon_move.category != MoveCategory::Status {
                len += 1;
                messages.push(Message::new(vec![format!("It was {}{}", effective, if effective == Effective::SuperEffective { "!" } else { "..." })], TextColor::White, Some(0.5)));
            }
            
        } else {
            warn!("Could not add move messages because text has not been initialized!");
        }
        len
    }

    fn add_item(&mut self, user: &PokemonInstance, item: ItemRef) -> usize {
        if let Some(messages) = self.text.messages.as_mut() {
            messages.push(
                Message::new(
                    vec![format!("A {} was used on {}", item.name, user.name())], 
                    TextColor::White, 
                    Some(0.5)
                )
            );
            1
        } else {
            warn!("Could not add item use messages because text has not been initialized!");
            0
        }
    }

    fn add_switch(&mut self, leaving: &PokemonInstance, coming: &PokemonInstance, player: bool) -> usize {
        if let Some(messages) = self.text.messages.as_mut() {
            if let Some(text) = if player { &mut self.player } else { &mut self.opponent } {
                text.active += 1;
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
            2
        } else {
            warn!("Could not add party switch messages because text has not been initialized!");
            0
        }
    }

    pub fn add_faint_text(&mut self, name: String) {
        if let Some(messages) = self.text.messages.as_mut() {
            self.faint = Some(Text::new(messages.len(), 1));
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

    pub fn on_move(&mut self, other_pokemon: &PokemonInstance, gui: &mut impl PokemonGui, is_player: bool) {

        gui.update_gui(other_pokemon, false);

        if other_pokemon.is_faint() {
            let player = self.player.as_ref().unwrap();
            let opponent = self.opponent.as_ref().unwrap();
            let pos = if is_player {
                player.pos
            } else {
                opponent.pos
            };
            if pos >= self.text.current_message() {
                if let Some(messages) = self.text.messages.as_mut() {
                    let len = if is_player {
                        player.len
                    } else {
                        opponent.len
                    };
                    for _ in 0..len {
                        messages.remove(pos);
                    }
                }
            }            
            self.add_faint_text(other_pokemon.name());
        }
    }

    pub fn perform_player(&self, battle: &Battle) -> bool {

        self.text.can_continue() && 
        battle.player.next_move_queued() && 
        !battle.player.active().is_faint() && 
        self.player.as_ref().map(|text| text.pos + text.active == self.text.current_message()).unwrap_or_default()
    }

    pub fn perform_opponent(&self, battle: &Battle) -> bool {
        self.text.can_continue() && 
        battle.opponent.next_move_queued() && 
        !battle.opponent.active().is_faint() && 
        self.opponent.as_ref().map(|text| text.pos + text.active == self.text.current_message()).unwrap_or_default()
    }

    pub fn perform_post(&self, battle: &Battle) -> bool {
        battle.post_run &&
        self.text.can_continue() &&
        !battle.player.next_move_queued() &&
        !battle.opponent.next_move_queued() &&
        self.post.as_ref().map(|text| text.pos + text.active == self.text.current_message()).unwrap_or_default()
    }

}