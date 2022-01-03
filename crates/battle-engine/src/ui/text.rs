use battle::pokemon::stat::{BattleStatType, Stage};
use pokedex::{
    ailment::Ailment,
    engine::{gui::MessageBox, math::vec2, text::MessagePage},
    item::Item,
    moves::Move,
    pokemon::{Experience, Level},
    types::Effective,
};

pub fn new() -> MessageBox {
    let mut messagebox = MessageBox::new(vec2(11.0, 11.0 + super::PANEL_Y), 1);
    messagebox.pages.reserve(6);
    messagebox
}

pub(crate) fn on_move(text: &mut MessageBox, pokemon_move: &Move, user: &str) {
    text.pages.push(MessagePage {
        lines: vec![format!("{} used {}!", user, pokemon_move.name)],
        wait: Some(0.5),
        color: MessagePage::WHITE,
    });
}

pub(crate) fn on_effective(text: &mut MessageBox, effective: &Effective) {
    if effective != &Effective::Effective {
        text.pages.push(MessagePage {
            lines: vec![format!(
                "It was {}{}",
                effective,
                if &Effective::SuperEffective == effective {
                    "!"
                } else {
                    "..."
                }
            )],
            wait: Some(0.5),
            color: MessagePage::WHITE,
        });
    }
}

pub(crate) fn on_crit(text: &mut MessageBox) {
    text.pages.push(MessagePage {
        lines: vec!["It was a critical hit!".to_owned()],
        wait: Some(0.5),
        color: MessagePage::WHITE,
    })
}

pub(crate) fn on_stat_stage(
    text: &mut MessageBox,
    pokemon: &str,
    stat: BattleStatType,
    stage: Stage,
) {
    text.pages.push(MessagePage {
        lines: vec![
            format!("{}'s {} was", pokemon, stat),
            format!(
                "{} by {}!",
                if stage.is_positive() {
                    "raised"
                } else {
                    "lowered"
                },
                stage.abs()
            ),
        ],
        wait: Some(0.5),
        color: MessagePage::WHITE,
    })
}

pub(crate) fn on_status(text: &mut MessageBox, pokemon: &str, status: Ailment) {
    text.pages.push(MessagePage {
        lines: vec![
            format!("{} was afflicted", pokemon),
            format!("with {:?}", status),
        ],
        wait: Some(0.5),
        color: MessagePage::WHITE,
    })
}

pub(crate) fn on_miss(text: &mut MessageBox, pokemon: &str) {
    text.pages.push(MessagePage {
        lines: vec![format!("{} missed!", pokemon)],
        wait: Some(0.5),
        color: MessagePage::WHITE,
    });
}

pub(crate) fn on_item(text: &mut MessageBox, target: &str, item: &Item) {
    text.pages.push(MessagePage {
        lines: vec![format!("A {} was used on {}", item.name, target,)],
        wait: Some(0.5),
        color: MessagePage::WHITE,
    });
}

fn on_leave(text: &mut MessageBox, leaving: &str) {
    text.pages.push(MessagePage {
        lines: vec![format!("Come back, {}!", leaving)],
        wait: Some(0.5),
        color: MessagePage::WHITE,
    });
}

pub(crate) fn on_switch(text: &mut MessageBox, leaving: &str, coming: &str) {
    on_leave(text, leaving);
    on_go(text, coming);
}

pub(crate) fn on_go(text: &mut MessageBox, coming: &str) {
    text.pages.push(MessagePage {
        lines: vec![format!("Go, {}!", coming)],
        wait: Some(0.5),
        color: MessagePage::WHITE,
    });
}

pub(crate) fn on_replace(text: &mut MessageBox, user: &str, coming: Option<&str>) {
    // if let Some(leaving) = leaving {
    //     on_leave(text, leaving);
    // }
    if let Some(coming) = coming {
        text.pages.push(MessagePage {
            lines: vec![format!("{} sent out {}!", user, coming)],
            wait: Some(0.5),
            color: MessagePage::WHITE,
        });
    }
}

pub(crate) fn on_faint(text: &mut MessageBox, is_wild: bool, is_player: bool, pokemon: &str) {
    text.pages.push(MessagePage {
        lines: vec![
            match is_player {
                true => pokemon.to_owned(),
                false => format!(
                    "{} {}",
                    match is_wild {
                        true => "Wild",
                        false => "Foe",
                    },
                    pokemon,
                ),
            },
            String::from("fainted!"),
        ],
        wait: Some(1.0),
        color: MessagePage::WHITE,
    });
}

pub(crate) fn on_catch(text: &mut MessageBox, pokemon: &str) {
    text.pages.push(MessagePage {
        lines: vec![String::from("Gotcha!"), format!("{} was caught!", pokemon)],
        wait: None,
        color: MessagePage::WHITE,
    });
}

pub(crate) fn on_gain_exp(
    text: &mut MessageBox,
    pokemon: &str,
    experience: Experience,
    level: Level,
) {
    text.pages.push(MessagePage {
        lines: vec![
            format!("{} gained {} EXP. points", pokemon, experience),
            format!("and {} levels!", level),
        ],
        wait: Some(1.0),
        color: MessagePage::WHITE,
    });
}

// pub(crate) fn on_level_up(text: &mut MessageBox, pokemon: &PokemonInstance, level: Level) {
//     text.pages.push(MessagePage::new(
//         vec![
//             format!("{} grew to", pokemon.name()),
//             format!("LV. {}!", level),
//         ],
//         Some(0.5),
//     ));
// }

pub(crate) fn on_fail(text: &mut MessageBox, lines: Vec<String>) {
    text.pages.push(MessagePage {
        lines,
        wait: Some(0.5),
        color: MessagePage::WHITE,
    });
}

pub(crate) fn on_flinch(text: &mut MessageBox, name: &str) {
    text.pages.push(MessagePage {
        lines: vec![format!("{} flinched!", name)],
        wait: Some(0.5),
        color: MessagePage::WHITE,
    });
}
