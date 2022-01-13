use battle::pokemon::stat::{BattleStatType, Stage};
use pokedex::{
    ailment::Ailment,
    engine::{
        graphics::Color,
        gui::MessageBox,
        math::vec2,
        text::{FontId, MessagePage, MessageState, TextColor},
        Context, EngineContext,
    },
    item::Item,
    moves::Move,
    pokemon::{Experience, Level},
    types::Effective,
};

pub struct BattleText {
    text: MessageBox,
    pub state: Option<MessageState<FontId, Color>>,
}

impl BattleText {
    pub fn new() -> Self {
        Self {
            text: MessageBox::new(vec2(11.0, 11.0 + super::PANEL_Y)),
            state: None,
        }
    }

    pub fn update(&mut self, ctx: &Context, eng: &EngineContext, delta: f32) {
        self.text.update(ctx, eng, delta, &mut self.state)
    }

    pub fn alive(&self) -> bool {
        self.state.is_some()
    }

    pub fn waiting(&self) -> bool {
        self.state.as_ref().map(|s| s.waiting).unwrap_or_default()
    }

    pub fn draw(&self, ctx: &mut Context, eng: &EngineContext) {
        self.text.draw(ctx, eng, self.state.as_ref())
    }

    pub fn page(&self) -> Option<usize> {
        self.state.as_ref().map(|s| s.page)
    }

    pub fn pages(&self) -> Option<usize> {
        self.state.as_ref().map(|s| s.pages())
    }

    pub(crate) fn on_move(&mut self, pokemon_move: &Move, user: &str) {
        let text = self.state.get_or_insert_with(|| MessageState::new(1, Default::default()));
        text.pages.push(MessagePage {
            lines: vec![format!("{} used {}!", user, pokemon_move.name)],
            wait: Some(0.5),
            color: TextColor::WHITE,
        });
    }

    pub(crate) fn on_effective(&mut self, effective: &Effective) {
        let text = self.state.get_or_insert_with(|| MessageState::new(1, Default::default()));
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
                color: TextColor::WHITE,
            });
        }
    }

    pub(crate) fn on_crit(&mut self) {
        let text = self.state.get_or_insert_with(|| MessageState::new(1, Default::default()));
        text.pages.push(MessagePage {
            lines: vec!["It was a critical hit!".to_owned()],
            wait: Some(0.5),
            color: TextColor::WHITE,
        })
    }

    pub(crate) fn on_stat_stage(&mut self, pokemon: &str, stat: BattleStatType, stage: Stage) {
        let text = self.state.get_or_insert_with(|| MessageState::new(1, Default::default()));
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
            color: TextColor::WHITE,
        })
    }

    pub(crate) fn on_status(&mut self, pokemon: &str, status: Ailment) {
        let text = self.state.get_or_insert_with(|| MessageState::new(1, Default::default()));
        text.pages.push(MessagePage {
            lines: vec![
                format!("{} was afflicted", pokemon),
                format!("with {:?}", status),
            ],
            wait: Some(0.5),
            color: TextColor::WHITE,
        })
    }

    pub(crate) fn on_miss(&mut self, pokemon: &str) {
        let text = self.state.get_or_insert_with(|| MessageState::new(1, Default::default()));
        text.pages.push(MessagePage {
            lines: vec![format!("{} missed!", pokemon)],
            wait: Some(0.5),
            color: TextColor::WHITE,
        });
    }

    pub(crate) fn on_item(&mut self, target: &str, item: &Item) {
        let text = self.state.get_or_insert_with(|| MessageState::new(1, Default::default()));
        text.pages.push(MessagePage {
            lines: vec![format!("A {} was used on {}", item.name, target,)],
            wait: Some(0.5),
            color: TextColor::WHITE,
        });
    }

    fn on_leave(&mut self, leaving: &str) {
        let text = self.state.get_or_insert_with(|| MessageState::new(1, Default::default()));
        text.pages.push(MessagePage {
            lines: vec![format!("Come back, {}!", leaving)],
            wait: Some(0.5),
            color: TextColor::WHITE,
        });
    }

    pub(crate) fn on_switch(&mut self, leaving: &str, coming: &str) {
        self.on_leave(leaving);
        self.on_go(coming);
    }

    pub(crate) fn on_go(&mut self, coming: &str) {
        let text = self.state.get_or_insert_with(|| MessageState::new(1, Default::default()));
        text.pages.push(MessagePage {
            lines: vec![format!("Go, {}!", coming)],
            wait: Some(0.5),
            color: TextColor::WHITE,
        });
    }

    pub(crate) fn on_replace(&mut self, user: &str, coming: Option<&str>) {
        // if let Some(leaving) = leaving {
        //     on_leave(text, leaving);
        // }
        if let Some(coming) = coming {
            let text = self.state.get_or_insert_with(|| MessageState::new(1, Default::default()));
            text.pages.push(MessagePage {
                lines: vec![format!("{} sent out {}!", user, coming)],
                wait: Some(0.5),
                color: TextColor::WHITE,
            });
        }
    }

    pub(crate) fn on_faint(&mut self, is_wild: bool, is_player: bool, pokemon: &str) {
        let text = self.state.get_or_insert_with(|| MessageState::new(1, Default::default()));
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
            color: TextColor::WHITE,
        });
    }

    pub(crate) fn on_catch(&mut self, pokemon: &str) {
        let text = self.state.get_or_insert_with(|| MessageState::new(1, Default::default()));
        text.pages.push(MessagePage {
            lines: vec![String::from("Gotcha!"), format!("{} was caught!", pokemon)],
            wait: None,
            color: TextColor::WHITE,
        });
    }

    pub(crate) fn on_gain_exp(&mut self, pokemon: &str, experience: Experience, level: Level) {
        let text = self.state.get_or_insert_with(|| MessageState::new(1, Default::default()));
        text.pages.push(MessagePage {
            lines: vec![
                format!("{} gained {} EXP. points", pokemon, experience),
                format!("and {} levels!", level),
            ],
            wait: Some(1.0),
            color: TextColor::WHITE,
        });
    }

    // pub(crate) fn on_level_up(&mut self, pokemon: &PokemonInstance, level: Level) {
    //     text.pages.push(MessagePage::new(
    //         vec![
    //             format!("{} grew to", pokemon.name()),
    //             format!("LV. {}!", level),
    //         ],
    //         Some(0.5),
    //     ));
    // }

    pub(crate) fn on_fail(&mut self, lines: Vec<String>) {
        let text = self.state.get_or_insert_with(|| MessageState::new(1, Default::default()));
        text.pages.push(MessagePage {
            lines,
            wait: Some(0.5),
            color: TextColor::WHITE,
        });
    }

    pub(crate) fn on_flinch(&mut self, name: &str) {
        let text = self.state.get_or_insert_with(|| MessageState::new(1, Default::default()));
        text.pages.push(MessagePage {
            lines: vec![format!("{} flinched!", name)],
            wait: Some(0.5),
            color: TextColor::WHITE,
        });
    }
}
