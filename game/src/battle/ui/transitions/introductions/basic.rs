use crate::{
    util::{Entity, Reset, Completable},
    play_sound,
    audio::Sound,
    macroquad::prelude::{Vec2, Texture2D, draw_texture_ex, WHITE, DrawTextureParams, Rect},
    text::MessagePage,
    gui::DynamicText,
    CRY_ID,
};

use crate::battle::{
    Battle,
    ui::{
        pokemon::status::PokemonStatusGui,
        transitions::BattleIntroduction,
    },
    pokemon::ActivePokemonArray,
};

pub struct BasicBattleIntroduction {
 
    player: Texture2D,
	counter: f32,
    offsets: (f32, f32),

}

impl Default for BasicBattleIntroduction {
    fn default() -> Self {
        Self {
            player: super::super::openers::player_texture(),
            counter: 0.0,
            offsets: Self::OFFSETS, // opponent, player
        }
    }
}

impl BasicBattleIntroduction {

    const OFFSETS: (f32, f32) = (-PokemonStatusGui::BATTLE_OFFSET, PokemonStatusGui::BATTLE_OFFSET);

    const PLAYER_T1: f32 = 42.0;
    const PLAYER_T2: f32 = Self::PLAYER_T1 + 18.0;
    const PLAYER_T3: f32 = Self::PLAYER_T2 + 18.0;
    const PLAYER_DESPAWN: f32 = 104.0;

    #[deprecated(note = "bad code, return vec of string (lines)")]
    pub(crate) fn concatenate(active: &ActivePokemonArray) -> String {
        let mut string = String::new();
        let len = active.len();
        for (index, active) in active.iter().enumerate() {
            if let Some(instance) = active.pokemon.as_ref() {
                if index != 0 {
                    if index == len - 2 {
                        string.push_str(", ");
                    } else if index == len - 1 {
                        string.push_str(" and ");
                    }
                }
                string.push_str(&instance.name());
            }
        }
        string
    }

    pub(crate) fn common_setup(&mut self, text: &mut DynamicText, active: &ActivePokemonArray) {        
        text.push(
            MessagePage::new(
                vec![
                    format!("Go! {}!", Self::concatenate(active)),
                ],
                Some(0.5),
            )
        );
    }

    pub(crate) fn render_player(&self, battle: &Battle) {
        if self.counter < Self::PLAYER_DESPAWN {
            draw_texture_ex(self.player, 41.0 + - self.counter, 49.0, WHITE, DrawTextureParams {
                source: Some(
                    Rect::new(
                        0.0, 
                        if self.counter >= Self::PLAYER_T3 { // 78.0
                            256.0
                        } else if self.counter >= Self::PLAYER_T2 { // 60.0
                            192.0
                        } else if self.counter >= Self::PLAYER_T1 { // 42.0
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
            for active in battle.player.active.iter() {
                active.renderer.render(Vec2::ZERO, WHITE);
            }
        }
    }

    pub(crate) fn render_opponent(&self, battle: &Battle) {
        for active in battle.opponent.active.iter() {
            active.renderer.render(Vec2::ZERO, WHITE);
            active.status.render(self.offsets.0, 0.0);
        }
    }

}

impl BattleIntroduction for BasicBattleIntroduction {

    fn spawn(&mut self, battle: &Battle, text: &mut DynamicText) {
        text.clear();
        text.push(
            MessagePage::new(
                vec![
                    format!("Wild {} appeared!", Self::concatenate(&battle.opponent.active))
                ],
                None,
            )
        );
        self.common_setup(text, &battle.player.active);
    }

    fn update(&mut self, delta: f32, battle: &mut Battle, text: &mut DynamicText) {

        text.update(delta);

        if text.current() + 1 == text.len() {
            if self.counter < Self::PLAYER_DESPAWN {
                self.counter += delta * 180.0;                
            }
        }

        if battle.opponent.active[0].status.alive() {
            if self.offsets.0 != 0.0 {
                self.offsets.0 += delta * 240.0;
                if self.offsets.0 > 0.0 {
                    self.offsets.0 = 0.0;
                }
            }
        } else if text.can_continue() && text.current() >= text.len() - 2 {
            for active in battle.opponent.active.iter_mut() {
                active.status.spawn();
                if let Some(instance) = active.pokemon.as_ref() {
                    play_sound(&Sound::variant(CRY_ID, Some(*instance.pokemon.id())));
                }
            }
            
        }

        if battle.player.active[0].renderer.spawner.spawning() {
            for active in battle.player.active.iter_mut() {
                active.renderer.spawner.update(delta);
            }
        } else if battle.player.active[0].status.alive() {
            if self.offsets.1 != 0.0 {
                self.offsets.1 -= delta * 240.0;
                if self.offsets.1 < 0.0 {
                    self.offsets.1 = 0.0;
                }
            }
        } else if self.counter >= Self::PLAYER_T2 {
            for active in battle.player.active.iter_mut() {
                active.renderer.spawn();
                active.status.spawn();
                if let Some(instance) = active.pokemon.as_ref() {
                    play_sound(&Sound::variant(CRY_ID, Some(*instance.pokemon.id())));
                }
            }
        }
        
    }

    fn render(&self, battle: &Battle) {
        self.render_opponent(battle);
        self.render_player(battle);
    }

}

impl Reset for BasicBattleIntroduction {
    fn reset(&mut self) {
        self.counter = 0.0;
        self.offsets = Self::OFFSETS;
    }
}
impl Completable for BasicBattleIntroduction {
    fn finished(&self) -> bool {
        self.counter >= Self::PLAYER_DESPAWN && self.offsets.1 == 0.0
    }
}