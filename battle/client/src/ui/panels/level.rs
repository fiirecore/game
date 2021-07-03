use game::{
    gui::TextDisplay,
    input::{pressed, Control},
    pokedex::{
        moves::MoveRef,
        pokemon::instance::PokemonInstance,
    },
    text::{MessagePage, TextColor},
    util::{Completable, Entity},
    tetra::Context,
};

use super::moves::MovePanel;

pub struct LevelUpMovePanel {
    
    state: LevelUpState,

    move_panel: MovePanel,

    moves: Vec<MoveRef>,
}

enum LevelUpState {
    NotAlive,
    Text,
    Moves,
}

impl LevelUpMovePanel {
    pub fn new(ctx: &mut Context) -> Self {
        Self {
            state: LevelUpState::NotAlive,
            move_panel: MovePanel::new(ctx),
            moves: Vec::new(),
        }
    }

    pub fn spawn(&mut self, instance: &PokemonInstance, text: &mut TextDisplay, moves: Vec<MoveRef>) {
        self.state = LevelUpState::Text;
        self.moves = moves;
        self.move_panel.update_names(instance);
        text.despawn();
    }

    pub fn update(&mut self, ctx: &Context, text: &mut TextDisplay, delta: f32, pokemon: &mut PokemonInstance) -> Option<(usize, MoveRef)> {
        match self.state {
            LevelUpState::Text => {
                match text.alive() {
                    true => {
                        text.update(ctx, delta);
                        if text.finished() {
                            self.state = LevelUpState::Moves;
                            text.despawn();
                        }
                        None
                    },
                    false => match self.moves.first() {
                        Some(move_ref) => {
                            text.spawn();
                            text.push(MessagePage::new(
                                vec![
                                    format!("{} is trying to", pokemon.name()),
                                    format!("learn {}", move_ref.name),
                                ],
                                None,
                            ));
                            self.update(ctx, text, delta, pokemon)
                        }
                        None => {
                            self.state = LevelUpState::NotAlive;
                            None
                        }
                    }
                }
            },
            LevelUpState::Moves => {
                self.move_panel.input(ctx);
                let a = pressed(ctx, Control::A);
                if pressed(ctx, Control::B) || a {
                    self.state = LevelUpState::Text;
                    let pokemon_move = self.moves.remove(0);
                    if a {
                        self.move_panel.names[self.move_panel.cursor] =
                        (pokemon_move, TextColor::Black);
                        pokemon.replace_move(self.move_panel.cursor, pokemon_move);
                        return Some((self.move_panel.cursor, pokemon_move));
                    }
                }
                None
            },
            LevelUpState::NotAlive => None,
        }
    }

    pub fn draw(&self, ctx: &mut Context) {
        match self.state {
            LevelUpState::Moves => self.move_panel.draw(ctx),
            LevelUpState::Text | LevelUpState::NotAlive => (),
        }
    }

    pub fn alive(&self) -> bool {
        !matches!(self.state, LevelUpState::NotAlive)
    }

}
