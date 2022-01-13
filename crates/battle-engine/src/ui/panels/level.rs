use core::ops::Deref;

use pokedex::{
    engine::{
        controls::{pressed, Control},
        text::{MessagePage, TextColor, MessageState},
        Context, EngineContext,
    },
    moves::{owned::OwnedMove, set::OwnedMoveSet, Move},
    pokemon::{owned::OwnablePokemon, Pokemon},
};

use crate::ui::text::BattleText;

use super::moves::MovePanel;

pub struct LevelUpMovePanel<M: Deref<Target = Move> + Clone> {
    state: LevelUpState,

    move_panel: MovePanel<M>,

    moves: Vec<M>,
}

enum LevelUpState {
    NotAlive,
    Text,
    Moves,
}

impl<M: Deref<Target = Move> + Clone> LevelUpMovePanel<M> {
    pub fn new() -> Self {
        Self {
            state: LevelUpState::NotAlive,
            move_panel: MovePanel::new(),
            moves: Vec::new(),
        }
    }

    pub fn spawn<P, MSET: Deref<Target = [OwnedMove<M>]>, I, G, N, H>(
        &mut self,
        instance: &OwnablePokemon<P, MSET, I, G, N, H>,
        moves: Vec<M>,
    ) {
        self.state = LevelUpState::Text;
        self.moves = moves;
        self.move_panel.update_names(instance);
    }

    pub fn update<P: Deref<Target = Pokemon>, I, G, N, H>(
        &mut self,
        ctx: &Context,
        eng: &EngineContext,
        text: &mut BattleText,
        delta: f32,
        pokemon: &mut OwnablePokemon<P, OwnedMoveSet<M>, I, G, N, H>,
    ) -> Option<(usize, M)> {
        match self.state {
            LevelUpState::Text => match text.alive() {
                true => {
                    text.update(ctx, eng, delta);
                    if !text.alive() {
                        self.state = LevelUpState::Moves;
                    }
                    None
                }
                false => match self.moves.first() {
                    Some(move_ref) => {
                        let state = text.state.get_or_insert_with(|| MessageState::new(1, Default::default()));
                        state.pages.push(MessagePage {
                            lines: vec![
                                format!("{} is trying to", pokemon.name()),
                                format!("learn {}", move_ref.name),
                            ],
                            wait: None,
                            color: TextColor::BLACK,
                        });
                        self.update(ctx, eng, text, delta, pokemon)
                    }
                    None => {
                        self.state = LevelUpState::NotAlive;
                        None
                    }
                },
            },
            LevelUpState::Moves => {
                self.move_panel.input(ctx, eng);
                let a = pressed(ctx, eng, Control::A);
                if pressed(ctx, eng, Control::B) || a {
                    self.state = LevelUpState::Text;
                    let pokemon_move = self.moves.remove(0);
                    if a {
                        self.move_panel.names[self.move_panel.cursor] =
                            Some((pokemon_move.clone(), TextColor::BLACK));
                        pokemon
                            .moves
                            .add(Some(self.move_panel.cursor), pokemon_move.clone());
                        return Some((self.move_panel.cursor, pokemon_move));
                    }
                }
                None
            }
            LevelUpState::NotAlive => None,
        }
    }

    pub fn draw(&self, ctx: &mut Context, eng: &EngineContext) {
        match self.state {
            LevelUpState::Moves => self.move_panel.draw(ctx, eng),
            LevelUpState::Text | LevelUpState::NotAlive => (),
        }
    }

    pub fn alive(&self) -> bool {
        !matches!(self.state, LevelUpState::NotAlive)
    }
}
