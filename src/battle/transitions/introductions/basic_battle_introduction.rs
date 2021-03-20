use firecore_pokedex::PokemonId;
use firecore_util::text::Message;
use firecore_util::text::MessageSet;
use firecore_util::Entity;
use firecore_util::text::TextColor;
use firecore_audio::{play_sound, Sound};
use macroquad::prelude::warn;

use crate::util::battle_data::TrainerData;
use crate::battle::battle::Battle;
use crate::battle::transitions::BattleIntroduction;
use crate::battle::transitions::BattleTransition;
use crate::gui::battle::battle_gui::BattleGui;
use crate::gui::battle::pokemon_gui::PokemonGui;
use crate::gui::GuiComponent;
use crate::util::graphics::draw_bottom;
use firecore_util::{Reset, Completable};

use crate::gui::dynamic_text::DynamicText;
use super::util::player_intro::PlayerBattleIntro;

pub struct BasicBattleIntroduction {

    alive: bool,
    finished: bool,

    player_id: PokemonId,
    opponent_id: PokemonId,

    pub text: DynamicText,
    pub player_intro: PlayerBattleIntro,

    finished_panel: bool,

}

impl BasicBattleIntroduction {

    pub fn new(panel_x: f32, panel_y: f32) -> Self {
        Self {
            alive: false,
            finished: false,

            player_id: 1,
            opponent_id: 1,

            text: DynamicText::new(11.0, 11.0, panel_x, panel_y),
            player_intro: PlayerBattleIntro::new(),
            
            finished_panel: false,
        }
    }

    pub fn common_setup(&mut self, battle: &Battle) {
        self.player_id = battle.player().pokemon.data.number;
        self.opponent_id = battle.opponent().pokemon.data.number;
    }

}

impl BattleTransition for BasicBattleIntroduction {

    fn on_start(&mut self) {
    }

    fn update(&mut self, delta: f32) {
        self.text.update(delta);
        if self.text.current_phrase() + 1 == self.text.text.len() as u8 {
            if !self.player_intro.is_finished() {
                self.player_intro.update(delta);                
            } else if self.text.timer.is_finished() {
                self.text.despawn();
                self.finished = true;
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

    fn input(&mut self, delta: f32) {
        self.text.input(delta);
    }

    fn setup(&mut self, battle: &Battle, _trainer_data: Option<&TrainerData>) {
        self.text.text = MessageSet { messages: vec![
            Message::with_color(vec![String::from("Wild ") + battle.opponent().pokemon.data.name.to_ascii_uppercase().as_str() + " appeared!"], true, TextColor::White),
            Message::with_color(vec![String::from("Go! ") + battle.player().pokemon.data.name.to_ascii_uppercase().as_str() + "!"], false, TextColor::White),
        ]};
        self.common_setup(battle);
    }

    fn render_offset(&self, battle: &Battle, offset: f32) {
        draw_bottom(battle.opponent_textures[battle.opponent_active], 144.0 - offset, 74.0);
        if !self.player_intro.is_finished() {
            self.player_intro.draw(offset);
        } else {
		    draw_bottom(battle.player_textures[battle.player_active], 40.0 + offset, 113.0);
        }        
    }

    fn update_gui(&mut self, battle_gui: &mut BattleGui, delta: f32) {
        if self.text.can_continue {
            if self.text.current_phrase() >= self.text.text.len() as u8 - 2 && !battle_gui.opponent_pokemon_gui.is_alive() {
                battle_gui.opponent_pokemon_gui.reset();
                battle_gui.opponent_pokemon_gui.spawn();
                if let Err(err) = play_sound(Sound::of("Cry", self.opponent_id)) {
                    warn!("Could not play opponent cry with error {}", err);
                }
            }
        }
        if self.player_intro.is_finished() && !battle_gui.player_pokemon_gui.is_alive() {
            battle_gui.player_pokemon_gui.reset();
            battle_gui.player_pokemon_gui.spawn();
            if let Err(err) = play_sound(Sound::of("Cry", self.player_id)) {
                warn!("Could not play opponent cry with error {}", err);
            }
        }
        if battle_gui.opponent_pokemon_gui.is_alive() {
            if battle_gui.opponent_pokemon_gui.panel.x + 5.0 < battle_gui.opponent_pokemon_gui.orig_x {
                battle_gui.opponent_pokemon_gui.offset_position(240.0 * delta, 0.0);
            } else if battle_gui.opponent_pokemon_gui.panel.x < battle_gui.opponent_pokemon_gui.orig_x {
                battle_gui.opponent_pokemon_gui.update_position(battle_gui.opponent_pokemon_gui.orig_x, battle_gui.opponent_pokemon_gui.panel.y);
            }
        }
        if battle_gui.player_pokemon_gui.is_alive() {
            //macroquad::prelude::info!("{}, {}", battle_gui.player_pokemon_gui.panel.x, battle_gui.player_pokemon_gui.orig_x);
            if battle_gui.player_pokemon_gui.panel.x - 5.0 > battle_gui.player_pokemon_gui.orig_x {
                battle_gui.player_pokemon_gui.offset_position(-240.0 * delta, 0.0);
            } else {
                battle_gui.player_pokemon_gui.update_position(battle_gui.player_pokemon_gui.orig_x, battle_gui.player_pokemon_gui.panel.y);
                self.finished_panel = true;
            }
        }
    }

}