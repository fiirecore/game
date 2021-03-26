use firecore_util::{
    Entity,
    Reset,
    Completable,
    text::{
        Message,
        TextColor,
    }
};

use firecore_audio::{play_sound, Sound};

use macroquad::prelude::{Vec2, warn};

use crate::battle::{
    Battle,
    gui::{
        BattleGui,
        pokemon::PokemonGui,
    },
    transitions::{
        BattleTransition,
        BattleIntroduction,
    }
};

use crate::util::graphics::draw_bottom;
use crate::gui::dynamic_text::DynamicText;

use super::util::player_intro::PlayerBattleIntro;

pub struct BasicBattleIntroduction {

    alive: bool,
    finished: bool,

    pub text: DynamicText,
    pub player_intro: PlayerBattleIntro,

    finished_panel: bool,

}

impl BasicBattleIntroduction {

    pub fn new(panel: Vec2) -> Self {
        Self {
            alive: false,
            finished: false,

            text: DynamicText::new(Vec2::new(11.0, 11.0), panel),
            player_intro: PlayerBattleIntro::new(),
            
            finished_panel: false,
        }
    }

}

impl BattleTransition for BasicBattleIntroduction {

    fn on_start(&mut self) {
    }

    fn update(&mut self, delta: f32) {
        self.text.update(delta);
        if let Some(messages) = self.text.messages.as_ref() {
            if self.text.current_message() + 1 == messages.len() {
                if !self.player_intro.is_finished() {
                    self.player_intro.update(delta);                
                } else if self.text.is_finished() {
                    self.text.despawn();
                    self.finished = true;
                }
            }
        }
        
	}

    fn render(&self) {
        self.text.render();
	}
    
}

impl Completable for BasicBattleIntroduction {

    fn is_finished(&self) -> bool {
        self.finished && self.player_intro.is_finished() && self.finished_panel
    }

}

impl Reset for BasicBattleIntroduction {

    fn reset(&mut self) {
        self.player_intro.reset();
        self.text.reset();
        self.finished_panel = false;
    }

}

impl Entity for BasicBattleIntroduction {

    fn spawn(&mut self) {
        self.alive = true;
        self.finished = false;
        self.text.spawn();
    }

    fn despawn(&mut self) {
        self.alive = false;
        self.finished = false;
        self.text.despawn();
    }

    fn is_alive(&self) -> bool {
        self.alive
    }

}

impl BattleIntroduction for BasicBattleIntroduction {

    fn input(&mut self) {
        self.text.input();
    }

    fn setup(&mut self, battle: &Battle) {
        self.text.messages = Some(vec![
            Message::new(
                vec![
                    format!("Wild {} appeared!", battle.opponent.active().pokemon.data.name.to_ascii_uppercase())
                ], 
                TextColor::White,
                None, 
            ),
            Message::new(
                vec![
                    format!("Go! {}!", battle.player.active().name())
                ],
                TextColor::White,
                Some(0.5),
            ),
        ]);
    }

    fn render_offset(&self, battle: &Battle, offset: f32) {
        draw_bottom(battle.opponent.active_texture(), 144.0 - offset, 74.0);
        if !self.player_intro.is_finished() {
            self.player_intro.draw(offset);
        } else {
		    draw_bottom(battle.player.active_texture(), 40.0 + offset, 113.0);
        }        
    }

    fn update_gui(&mut self, battle: &Battle, battle_gui: &mut BattleGui, delta: f32) {
        if self.text.can_continue {
            if let Some(messages) = self.text.messages.as_ref() {
                if self.text.current_message() >= messages.len() - 2 && !battle_gui.opponent_pokemon_gui.is_alive() {
                    battle_gui.opponent_pokemon_gui.reset();
                    battle_gui.opponent_pokemon_gui.spawn();
                    if let Err(err) = play_sound(Sound::of("Cry", battle.opponent.active().pokemon.data.id)) {
                        warn!("Could not play opponent cry with error {}", err);
                    }
                }
            }            
        }
        if self.player_intro.is_finished() && !battle_gui.player_pokemon_gui.is_alive() {
            battle_gui.player_pokemon_gui.reset();
            battle_gui.player_pokemon_gui.spawn();
            if let Err(err) = play_sound(Sound::of("Cry", battle.player.active().pokemon.data.id)) {
                warn!("Could not play opponent cry with error {}", err);
            }
        }
        if battle_gui.opponent_pokemon_gui.is_alive() {
            if battle_gui.opponent_pokemon_gui.pos.x + 5.0 < battle_gui.opponent_pokemon_gui.orig_x {
                battle_gui.opponent_pokemon_gui.offset_position(240.0 * delta, 0.0);
            } else if battle_gui.opponent_pokemon_gui.pos.x < battle_gui.opponent_pokemon_gui.orig_x {
                battle_gui.opponent_pokemon_gui.update_position(battle_gui.opponent_pokemon_gui.orig_x, battle_gui.opponent_pokemon_gui.pos.y);
            }
        }
        if battle_gui.player_pokemon_gui.is_alive() {
            if battle_gui.player_pokemon_gui.pos.x - 5.0 > battle_gui.player_pokemon_gui.orig_x {
                battle_gui.player_pokemon_gui.offset_position(-240.0 * delta, 0.0);
            } else {
                battle_gui.player_pokemon_gui.update_position(battle_gui.player_pokemon_gui.orig_x, battle_gui.player_pokemon_gui.pos.y);
                self.finished_panel = true;
            }
        }
    }

}