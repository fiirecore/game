use firecore_data::player::list::PlayerSaves;
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

use macroquad::prelude::{Vec2, warn, Texture2D, draw_texture_ex, WHITE, DrawTextureParams, Rect};

use crate::battle::manager::TrainerTextures;
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

use crate::util::graphics::{byte_texture, draw_bottom};
use crate::gui::text::DynamicText;
use crate::util::text::process_messages;

pub struct BasicBattleIntroduction {

    alive: bool,
    finished: bool,

    pub text: DynamicText,
    
    player: Texture2D,
	counter: f32,

    finished_panel: bool,

}

impl BasicBattleIntroduction {

    pub fn new(panel: Vec2) -> Self {
        Self {
            alive: false,
            finished: false,

            text: DynamicText::new(Vec2::new(11.0, 11.0), panel),

            player: byte_texture(include_bytes!("../../../../build/assets/battle/player.png")),
			counter: 0.0,
            
            finished_panel: false,
        }
    }

    pub fn common_setup(&mut self, battle: &Battle) {
        if let Some(messages) = self.text.messages.as_mut() {
            messages.push(
                Message::new(
                    vec![
                        format!("Go! {}!", battle.player.active().name())
                    ],
                    TextColor::White,
                    Some(0.5),
                )
            );
            if let Some(saves) = macroquad::prelude::collections::storage::get::<PlayerSaves>() {
                process_messages(saves.get(), messages);
            }
            
        }
    }

    pub fn render_player(&self, battle: &Battle, offset: f32) {
        if self.counter < 104.0 {
            draw_texture_ex(self.player, 41.0 + offset - self.counter, 49.0, WHITE, DrawTextureParams {
                source: Some(
                    Rect::new(
                        0.0, 
                        if self.counter >= 78.0 {
                            256.0
                        } else if self.counter >= 60.0 {
                            192.0
                        } else if self.counter >= 42.0 {
                            128.0
                        } else if self.counter > 0.0 {
                            64.0
                        } else {
                            0.0
                        }, 
                        64.0, 
                        64.0
                    )
                ),
                ..Default::default()
            });
        } else {
            draw_bottom(battle.player.active_texture(), 40.0 + offset, 113.0);
        }
    }

}

impl BattleIntroduction for BasicBattleIntroduction {

    fn input(&mut self) {
        self.text.input();
    }

    fn setup(&mut self, battle: &Battle, _trainer_sprites: &TrainerTextures) {
        self.text.messages = Some(vec![
            Message::new(
                vec![
                    format!("Wild {} appeared!", battle.opponent.active().pokemon.data.name.to_ascii_uppercase())
                ], 
                TextColor::White,
                None, 
            ),
        ]);
        self.common_setup(battle);
    }

    fn render_offset(&self, battle: &Battle, offset: f32) {
        draw_bottom(battle.opponent.active_texture(), 144.0 - offset, 74.0);
        self.render_player(battle, offset);
    }

    fn update_gui(&mut self, battle: &Battle, battle_gui: &mut BattleGui, delta: f32) {
        if self.text.can_continue {
            if let Some(messages) = self.text.messages.as_ref() {
                if self.text.current_message() >= messages.len() - 2 && !battle_gui.opponent.is_alive() {
                    battle_gui.opponent.reset();
                    battle_gui.opponent.spawn();
                    if let Err(err) = play_sound(Sound::of("Cry", battle.opponent.active().pokemon.data.id)) {
                        warn!("Could not play opponent cry with error {}", err);
                    }
                }
            }            
        }
        if self.counter >= 104.0 && !battle_gui.player.is_alive() {
            battle_gui.player.reset();
            battle_gui.player.spawn();
            if let Err(err) = play_sound(Sound::of("Cry", battle.player.active().pokemon.data.id)) {
                warn!("Could not play opponent cry with error {}", err);
            }
        }
        if battle_gui.opponent.is_alive() {
            if battle_gui.opponent.pos.x + 5.0 < battle_gui.opponent.orig_x {
                battle_gui.opponent.offset_position(240.0 * delta, 0.0);
            } else if battle_gui.opponent.pos.x < battle_gui.opponent.orig_x {
                battle_gui.opponent.update_position(battle_gui.opponent.orig_x, battle_gui.opponent.pos.y);
            }
        }
        if battle_gui.player.is_alive() {
            if battle_gui.player.pos.x - 5.0 > battle_gui.player.orig_x {
                battle_gui.player.offset_position(-240.0 * delta, 0.0);
            } else {
                battle_gui.player.update_position(battle_gui.player.orig_x, battle_gui.player.pos.y);
                self.finished_panel = true;
            }
        }
    }

}

impl BattleTransition for BasicBattleIntroduction {

    fn on_start(&mut self) {}

    fn update(&mut self, delta: f32) {
        self.text.update(delta);
        if let Some(messages) = self.text.messages.as_ref() {
            if self.text.current_message() + 1 == messages.len() {
                if self.counter < 104.0 {
                    self.counter += delta * 180.0;                
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
        self.finished && self.counter >= 104.0 && self.finished_panel
    }

}

impl Reset for BasicBattleIntroduction {

    fn reset(&mut self) {
        self.counter = 0.0;
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