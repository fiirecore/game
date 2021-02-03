use crate::entity::Entity;
use crate::io::data::text::Message;
use crate::io::data::text::color::TextColor;
use crate::util::battle_data::TrainerData;
use crate::util::{Update, Render};
use crate::battle::battle::Battle;
use crate::battle::transitions::BattleIntroduction;
use crate::battle::transitions::BattleTransition;
use crate::gui::battle::battle_gui::BattleGui;
use crate::gui::battle::pokemon_gui::PokemonGui;
use crate::gui::Activatable;
use crate::gui::GuiComponent;
use crate::util::graphics::draw_bottom;
use crate::util::{Reset, Completable};
use crate::util::Load;

use crate::gui::dynamic_text::DynamicText;
use super::util::player_intro::PlayerBattleIntro;

pub struct BasicBattleIntroduction {

    alive: bool,
    finished: bool,

    pub intro_text: DynamicText,
    pub player_intro: PlayerBattleIntro,

    finished_panel: bool,

}

impl BasicBattleIntroduction {

    pub fn new(panel_x: f32, panel_y: f32) -> Self {

        Self {

            alive: false,
            finished: false,

            intro_text: DynamicText::new(11.0, 11.0, panel_x, panel_y),
            player_intro: PlayerBattleIntro::new(),
            
            finished_panel: false,

        }

    }

}

impl Reset for BasicBattleIntroduction {

    fn reset(&mut self) {
        self.intro_text.load();
        self.player_intro.reset();
        self.finished_panel = false;
    }

}

impl Load for BasicBattleIntroduction {

    fn load(&mut self) {
        self.player_intro.load();
    }

    fn on_start(&mut self) {
        self.player_intro.on_start();
    }

}

impl Completable for BasicBattleIntroduction {

    fn is_finished(&self) -> bool {
        self.finished && !self.player_intro.should_update() && self.finished_panel
    }

}

impl Entity for BasicBattleIntroduction {

    fn spawn(&mut self) {
        self.alive = true;
        self.finished = false;
        self.intro_text.enable();
        self.intro_text.focus();
    }

    fn despawn(&mut self) {
        self.alive = false;
        self.finished = false;
        self.intro_text.disable();
    }

    fn is_alive(&self) -> bool {
        self.alive
    }
}

impl Update for BasicBattleIntroduction {

    fn update(&mut self, delta: f32) {
        self.intro_text.update(delta);
        if self.intro_text.next() + 1 == self.intro_text.text.len() as u8 {
            if self.player_intro.should_update() {
                self.player_intro.update(delta);                
            } else if self.intro_text.timer.is_finished() {
                self.intro_text.disable();
                self.finished = true;
            }
        }
	}
}

impl Render for BasicBattleIntroduction {

    fn render(&self) {
        self.intro_text.render();
	}

}

impl BattleIntroduction for BasicBattleIntroduction {

    fn input(&mut self, delta: f32) {
        self.intro_text.input(delta);
    }

    fn setup(&mut self, battle: &Battle, _trainer_data: Option<&TrainerData>) {
        self.intro_text.text = crate::io::data::text::MessageSet { messages: vec![
            Message::with_color(vec![String::from("Wild ") + battle.opponent().data.name.to_uppercase().as_str() + " appeared!"], false, TextColor::White),
            Message::with_color(vec![String::from("Go! ") + battle.player().data.name.to_uppercase().as_str() + "!"], true, TextColor::White),
        ]};
    }

    fn render_offset(&self, battle: &Battle, offset: f32) {
        draw_bottom(battle.opponent_textures[battle.opponent_active], 144.0 - offset, 74.0);
        if self.player_intro.should_update() {
            self.player_intro.draw(offset);
        } else {
		    draw_bottom(battle.player_textures[battle.player_active], 40.0 + offset, 113.0);
        }        
    }

    fn update_gui(&mut self, battle_gui: &mut BattleGui, delta: f32) {
        if self.intro_text.can_continue {
            if self.intro_text.next() >= self.intro_text.text.len() as u8 - 2 && !battle_gui.opponent_pokemon_gui.is_alive() {
                battle_gui.opponent_pokemon_gui.reset();
                battle_gui.opponent_pokemon_gui.spawn();
            }
        }
        if !self.player_intro.should_update() && !battle_gui.player_pokemon_gui.is_alive() {
            battle_gui.player_pokemon_gui.reset();
            battle_gui.player_pokemon_gui.spawn();
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

impl BattleTransition for BasicBattleIntroduction {}