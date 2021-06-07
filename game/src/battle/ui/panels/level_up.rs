use game::{
    util::{
        Entity,
        Completable,
    },
    pokedex::{
        moves::{
            MoveRef,
            instance::MoveInstance,
        },
        pokemon::instance::PokemonInstance,
    },
    text::{MessagePage, TextColor},
    input::{pressed, Control},
    macroquad::prelude::Vec2,
    gui::text::DynamicText,
};

use super::moves::MovePanel;

pub struct LevelUpMovePanel {

    alive: bool,

    text: DynamicText,
    move_panel: MovePanel,

    name: String,
    moves: Vec<MoveRef>,

    moves_active: bool,

}

impl LevelUpMovePanel {

    pub fn new(panel: Vec2) -> Self {
        Self {
            alive: false,
            text: DynamicText::new(Vec2::new(11.0, 11.0), panel, 1, &TextColor::White, 1, "levelup"),
            move_panel: MovePanel::new(panel),
            name: String::new(),
            moves: Vec::new(),
            moves_active: false,
        }
    }

    pub fn spawn(&mut self, index: usize, instance: &PokemonInstance, moves: Vec<MoveRef>) {
        self.alive = true;
        self.moves = moves;
        self.name = instance.name();
        self.move_panel.update_names(instance);
    }

    pub fn update(&mut self, delta: f32, pokemon: &mut PokemonInstance) {
        if !self.moves_active {
            if self.text.alive() {
                self.text.input();
                self.text.update(delta);
                if self.text.finished() {
                    self.moves_active = true;
                    self.text.despawn();
                }
            } else {
                self.text.spawn();
                self.text.push(
                    MessagePage::new(
                        vec![
                            format!("{} is trying to", self.name),
                            format!("learn {}", self.moves[0].name)
                        ], 
                        None,
                    )
                );
            }
        } else {
            self.move_panel.input();
            if pressed(Control::A) {
                let pokemon_move = self.moves.remove(0);
                self.move_panel.names[self.move_panel.cursor] = pokemon_move.name.to_ascii_uppercase();
                pokemon.moves[self.move_panel.cursor] = MoveInstance::new(pokemon_move);
                self.next();
            }
            if pressed(Control::B) {
                self.moves.remove(0);
                self.next();
            }
        }
    }

    pub fn render(&self) {
        if self.alive {
            if !self.moves_active {
                self.text.render()
            } else {
                self.move_panel.render()
            }
        }
    }

    fn next(&mut self) {
        if self.moves.is_empty() {
            self.despawn();
        } else {
            self.moves_active = false;
        }
    }

}