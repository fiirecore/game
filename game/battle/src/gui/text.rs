use game::{
    text::TextColor,
    pokedex::{
        pokemon::{
            instance::PokemonInstance,
            types::effective::Effective,
        },
        moves::{
            MoveRef,
            MoveCategory,
        },
        item::ItemRef,
    },
    text::MessagePage,
    gui::text::DynamicText,
    macroquad::prelude::Vec2,
};

pub fn new() -> DynamicText {
    DynamicText::new(Vec2::new(11.0, 11.0), Vec2::new(0.0, 113.0), 1, TextColor::White, 6, "btlmoves")
}

pub fn on_move(text: &mut DynamicText, pokemon_move: MoveRef, user: &PokemonInstance, target: &PokemonInstance) {

    text.push(
        MessagePage::new(
            vec![user.name() + " used " + &pokemon_move.name + "!"],
            Some(0.5),
        )
    );

}

pub fn on_item(text: &mut DynamicText, user: &PokemonInstance, item: ItemRef) {
    text.push(
        MessagePage::new(
            vec![format!("A {} was used on {}", item.name, user.name())], 
            Some(0.5)
        )
    );
}

pub fn on_switch(text: &mut DynamicText, leaving: &PokemonInstance, coming: &PokemonInstance) {
    text.push(
        MessagePage::new(
            vec![format!("Come back, {}!", leaving.name())],
            Some(0.5),
        )
    );
    on_go(text, coming);
}

pub fn on_go(text: &mut DynamicText, coming: &PokemonInstance) {
    text.push(
        MessagePage::new(
            vec![format!("Go, {}!", coming.name())],
            Some(0.5),
        )
    );
}

// #[deprecated(note = "todo")]
fn add_persistent_move(text: &mut DynamicText, persistent: &game::pokedex::moves::persistent::PersistentMoveInstance, target: &PokemonInstance) {
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

pub fn on_faint(text: &mut DynamicText, name: String) {
    text.push(
        MessagePage::new(
            vec![name + " fainted!"],
            Some(1.0), 
        )            
    );
}

pub(crate) fn player_gain_exp(text: &mut DynamicText, name: String, exp: u32, level: Option<u8>) {
    text.push(
        MessagePage::new(
            vec![
                format!("{} gained", name),
                format!("{} EXP. points!", exp),
            ],
            None,
        )
    );
    if let Some(level) = level {
        text.push(
            MessagePage::new(
                vec![
                    name + " grew to",
                    format!("LV. {}!", level),
                ],
                Some(0.5),
            )
        );                
    }     
}