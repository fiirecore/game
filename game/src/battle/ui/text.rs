use crate::{
    text::{MessagePage, TextColor},
    gui::TextDisplay,
    tetra::math::Vec2,
    pokedex::{
        types::Effective,
        pokemon::{
            Level,
            instance::PokemonInstance,
            stat::{StatType, Stage},
        },
        moves::{
            Move,
        },
        item::Item,
    },
};

use crate::battle::pokemon::view::PokemonView;

pub fn new() -> TextDisplay {
    TextDisplay::new(super::PANEL_ORIGIN.position + Vec2::new(11.0, 11.0), 1, TextColor::White, 6)
}

pub(crate) fn on_move(text: &mut TextDisplay, pokemon_move: &Move, user: &dyn PokemonView) {
    text.push(
        MessagePage::new(
            vec![format!("{} used {}!", user.name(), pokemon_move.name)],
            Some(0.5),
        )
    );
}

pub(crate) fn on_effective(text: &mut TextDisplay, effective: &Effective) {
    if effective != &Effective::Effective {
        text.push(
            MessagePage::new(
                vec![format!("It was {}{}", effective, if &Effective::SuperEffective == effective { "!" } else { "..." })], 
                Some(0.5),
            )
        );
    }
}

pub(crate) fn on_stat_stage(text: &mut TextDisplay, pokemon: &dyn PokemonView, stat: StatType, stage: Stage) {
    text.push(
        MessagePage::new(
            vec![
                format!("{}'s {:?} was", pokemon.name(), stat),
                format!("{} by {}!", if stage.is_positive() { "raised" } else { "lowered" }, stage.abs())
                ],
            Some(0.5)
        )
    )
}

pub(crate) fn on_miss(text: &mut TextDisplay, pokemon: &dyn PokemonView) {
    text.push(
        MessagePage::new(
            vec![format!("{} missed!", pokemon.name())], 
            Some(0.5)
        )
    );
}

pub(crate) fn on_item(text: &mut TextDisplay, pokemon: Option<&dyn PokemonView>, item: &Item) {
    text.push(
        MessagePage::new(
            vec![format!("A {} was used on {}", item.name, pokemon.map(|p| p.name()).unwrap_or("None"))], 
            Some(0.5)
        )
    );
}

fn on_leave(text: &mut TextDisplay, leaving: &dyn PokemonView) {
    text.push(
        MessagePage::new(
            vec![format!("Come back, {}!", leaving.name())],
            Some(0.5),
        )
    );
}

pub(crate) fn on_switch(text: &mut TextDisplay, leaving: &dyn PokemonView, coming: &dyn PokemonView) {
    on_leave(text, leaving);
    on_go(text, coming);
}

pub(crate) fn on_go(text: &mut TextDisplay, coming: &dyn PokemonView) {
    text.push(
        MessagePage::new(
            vec![format!("Go, {}!", coming.name())],
            Some(0.5),
        )
    );
}

pub(crate) fn on_replace(text: &mut TextDisplay, user: &str, coming: Option<&dyn PokemonView>) {
    // if let Some(leaving) = leaving {
    //     on_leave(text, leaving);
    // }
    if let Some(coming) = coming {
        text.push(
            MessagePage::new(
                vec![format!("{} sent out {}!", user, coming.name())],
                Some(0.5),
            )
        );
    }
}

// #[deprecated(note = "todo")]
// pub(crate) fn add_persistent_move(text: &mut TextDisplay, persistent: &game::pokedex::moves::persistent::PersistentMoveInstance, target: &PokemonInstance) {
//     text.push(MessagePage::new(match persistent.actions {
//         game::pokedex::moves::script::MoveActionType::Damage(..) => {
//             vec![format!("{} was hurt by {}!", target.name(), persistent.pokemon_move.value().name)]
//         }
//         game::pokedex::moves::script::MoveActionType::Status(.., effect) => {
//             vec![format!("{} was afflicted by {:?}!", target.name(), effect)]
//         }
//         game::pokedex::moves::script::MoveActionType::Drain(..) => {
//             vec![format!("{} was drained by {}!", target.name(), persistent.pokemon_move.value().name)]
//         }
//     }, None));
// }

pub(crate) fn on_faint(text: &mut TextDisplay, is_wild: bool, is_player: bool, pokemon: &dyn PokemonView) {
    text.push(
        MessagePage::new(
            vec![
                match is_player {
                    true => pokemon.name().to_string(),
                    false => format!("{} {}",
                        match is_wild {
                            true => "Wild",
                            false => "Foe",
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

pub(crate) fn on_catch(text: &mut TextDisplay, pokemon: Option<&PokemonInstance>) {
    text.push(match pokemon {
        Some(pokemon) => MessagePage::new(
            vec![
                String::from("Gotcha!"),
                format!("{} was caught!", pokemon.name())
            ], 
            None
        ),
        None => MessagePage::new(
            vec![
                String::from("Could not catch pokemon!"),                
            ], 
            Some(2.0)
        ),
    });
}

pub(crate) fn on_gain_exp(text: &mut TextDisplay, pokemon: &PokemonInstance, exp: u32) {
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

pub(crate) fn on_level_up(text: &mut TextDisplay, pokemon: &PokemonInstance, level: Level) {
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

pub(crate) fn on_fail(text: &mut TextDisplay, lines: Vec<String>) {
    text.push(
        MessagePage::new(
            lines,
            Some(0.5),
        )
    );  
}