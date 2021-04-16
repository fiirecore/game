use game::{
    util::{
        Entity,
        Completable,
        text::{
            Message,
            TextColor
        },
    },
    pokedex::{
        moves::{
            MoveRef,
            instance::MoveInstance,
        },
        pokemon::instance::PokemonInstance,
    },
    input::{pressed, Control},
    macroquad::prelude::Vec2,
    gui::text::DynamicText,
};

use super::moves::MovePanel;

pub struct LevelUpMovePanel {
 
    // pub wants_to_spawn: bool,

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
            // wants_to_spawn: false,
            alive: false,
            text: DynamicText::new(Vec2::new(11.0, 11.0), panel),
            move_panel: MovePanel::new(panel),
            name: String::new(),
            moves: Vec::new(),
            moves_active: false,
        }
    }

    pub fn setup(&mut self, instance: &PokemonInstance, moves: Vec<MoveRef>) {
        self.moves = moves;
        self.name = instance.name();
        self.move_panel.update_names(instance);
    }

    pub fn update(&mut self, delta: f32, pokemon: &mut PokemonInstance) {
        if !self.moves_active {
            if self.text.is_alive() {
                self.text.input();
                self.text.update(delta);
                if self.text.is_finished() {
                    self.moves_active = true;
                    self.text.despawn();
                }
            } else {
                self.text.spawn();
                self.text.messages = Some(
                    vec![
                        Message::new(
                            vec![
                                format!("{} is trying to", self.name),
                                format!("learn {}", self.moves[0].name)
                            ], 
                            TextColor::White,
                            None,
                        )
                    ]
                )
            }
        } else {
            self.move_panel.input();
            if pressed(Control::A) {
                let pokemon_move = self.moves.remove(0);
                self.move_panel.move_names[self.move_panel.cursor] = pokemon_move.name.to_ascii_uppercase();
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

impl Entity for LevelUpMovePanel {
    fn spawn(&mut self) {
        self.alive = true;
        // self.wants_to_spawn = false;
    }

    fn despawn(&mut self) {
        self.alive = false;
    }

    fn is_alive(&self) -> bool {
        self.alive
    }
}