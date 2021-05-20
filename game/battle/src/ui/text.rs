use game::pokedex::pokemon::Level;
use game::{
    text::{MessagePage, TextColor},
    gui::DynamicText,
    macroquad::prelude::Vec2,
    pokedex::{
        pokemon::{
            instance::PokemonInstance,
            types::Effective,
        },
        moves::Move,
        item::Item,
    },
    battle::BattleTeam,
};

use crate::BattleType;

pub fn new() -> DynamicText {
    DynamicText::new(super::PANEL_ORIGIN + Vec2::new(11.0, 11.0), 1, TextColor::White, 6)
}

pub(crate) fn on_move(text: &mut DynamicText, pokemon_move: &Move, user: &PokemonInstance) {
    text.push(
        MessagePage::new(
            vec![format!("{} used {}!", user.name(), pokemon_move.name)],
            Some(0.5),
        )
    );
}

pub(crate) fn on_effective(text: &mut DynamicText, effective: &Effective) {
    text.push(
        MessagePage::new(
            vec![format!("It was {}{}", effective, if Effective::SuperEffective.eq(effective) { "!" } else { "..." })], 
            Some(0.5),
        )
    );
}

pub(crate) fn on_item(text: &mut DynamicText, pokemon: &PokemonInstance, item: &Item) {
    text.push(
        MessagePage::new(
            vec![format!("A {} was used on {}", item.name, pokemon.name())], 
            Some(0.5)
        )
    );
}

pub(crate) fn on_switch(text: &mut DynamicText, leaving: &PokemonInstance, coming: &PokemonInstance) {
    text.push(
        MessagePage::new(
            vec![format!("Come back, {}!", leaving.name())],
            Some(0.5),
        )
    );
    on_go(text, coming);
}

pub(crate) fn on_go(text: &mut DynamicText, coming: &PokemonInstance) {
    text.push(
        MessagePage::new(
            vec![format!("Go, {}!", coming.name())],
            Some(0.5),
        )
    );
}

// #[deprecated(note = "todo")]
pub(crate) fn add_persistent_move(text: &mut DynamicText, persistent: &game::pokedex::moves::persistent::PersistentMoveInstance, target: &PokemonInstance) {
    text.push(MessagePage::new(match persistent.actions {
        game::pokedex::moves::script::MoveActionType::Damage(..) => {
            vec![format!("{} was hurt by {}!", target.name(), persistent.pokemon_move.value().name)]
        }
        game::pokedex::moves::script::MoveActionType::Status(.., effect) => {
            vec![format!("{} was afflicted by {:?}!", target.name(), effect)]
        }
        game::pokedex::moves::script::MoveActionType::Drain(..) => {
            vec![format!("{} was drained by {}!", target.name(), persistent.pokemon_move.value().name)]
        }
    }, None));
}

pub(crate) fn on_faint(text: &mut DynamicText, battle_type: BattleType, team: BattleTeam, pokemon: &PokemonInstance) {
    text.push(
        MessagePage::new(
            vec![
                match team {
                    BattleTeam::Player => pokemon.name().to_string(),
                    BattleTeam::Opponent => format!("{} {}",
                        match battle_type {
                            BattleType::Wild => "Wild",
                            _ => "Foe",
                        },
                        pokemon.name(),
                    ),
                },
                String::from("fainted!"),
            ],            
            Some(1.0), 
        )            
    );
}

pub(crate) fn on_catch(text: &mut DynamicText, target: &PokemonInstance) {
    text.push(
        MessagePage::new(
            vec![
                format!("{} used", game::storage::data().name),
                String::from("Pokeball!")
            ], 
            Some(2.0)
        )
    );
    text.push(
        MessagePage::new(
            vec![
                String::from("Gotcha!"),
                format!("{} was caught!", target.name())
            ], 
            None
        )
    )
}

pub(crate) fn on_gain_exp(text: &mut DynamicText, pokemon: &PokemonInstance, exp: u32) {
    text.push(
        MessagePage::new(
            vec![
                format!("{} gained", pokemon.name()),
                format!("{} EXP. points!", exp),
            ],
            Some(1.0),
        )
    );   
}

pub(crate) fn on_level_up(text: &mut DynamicText, pokemon: &PokemonInstance, level: Level) {
    text.push(
        MessagePage::new(
            vec![
                format!("{} grew to", pokemon.name()),
                format!("LV. {}!", level),
            ],
            Some(0.5),
        )
    );  
}

pub(crate) fn on_fail(text: &mut DynamicText, message: String) {
    text.push(
        MessagePage::new(
            vec![message],
            Some(0.5),
        )
    );  
}