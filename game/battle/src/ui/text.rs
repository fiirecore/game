use game::{
    util::battle::BattleType,
    text::TextColor,
    pokedex::{
        pokemon::{
            instance::PokemonInstance,
            types::Effective,
        },
        moves::{
            MoveRef,
        },
        item::ItemRef,
    },
    text::MessagePage,
    gui::DynamicText,
    macroquad::prelude::Vec2,
};

pub fn new() -> DynamicText {
    DynamicText::new(super::PANEL_ORIGIN + Vec2::new(11.0, 11.0), 1, TextColor::White, 6)
}

pub(crate) fn on_move(text: &mut DynamicText, pokemon_move: MoveRef, user: &PokemonInstance) {
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
            Some(0.5)
        )
    );
}

pub(crate) fn on_item(text: &mut DynamicText, pokemon: &PokemonInstance, item: ItemRef) {
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
    match persistent.actions {
        game::pokedex::moves::script::MoveActionType::Damage(..) => {
            text.push(MessagePage::new(vec![format!("{} was hurt by {}!", target.name(), persistent.pokemon_move.name)], None));
        }
        game::pokedex::moves::script::MoveActionType::Status(.., effect) => {
            text.push(MessagePage::new(vec![format!("{} was afflicted by {:?}!", target.name(), effect)], None));
        }
        game::pokedex::moves::script::MoveActionType::Drain(..) => {
            text.push(MessagePage::new(vec![format!("{} was drained by {}!", target.name(), persistent.pokemon_move.name)], None));
        }
    }
}

pub(crate) fn on_faint(text: &mut DynamicText, battle_type: BattleType, pokemon: &PokemonInstance) {
    text.push(
        MessagePage::new(
            vec![
                format!("{} {}",
                    match battle_type {
                        BattleType::Wild => "Wild",
                        _ => "Foe",
                    },
                    pokemon.name()
                ),
                String::from("fainted!"),
            ],            
            Some(1.0), 
        )            
    );
}

pub(crate) fn on_gain_exp(text: &mut DynamicText, pokemon: &PokemonInstance, exp: u32) {
    text.push(
        MessagePage::new(
            vec![
                format!("{} gained", pokemon.name()),
                format!("{} EXP. points!", exp),
            ],
            None,
        )
    );   
}

pub(crate) fn on_level_up(text: &mut DynamicText, pokemon: &PokemonInstance) {
    text.push(
        MessagePage::new(
            vec![
                format!("{} grew to", pokemon.name()),
                format!("LV. {}!", pokemon.data.level),
            ],
            Some(0.5),
        )
    );  
}