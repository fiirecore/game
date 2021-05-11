use game::{
    util::{Entity, Reset, Completable},
    play_sound,
    audio::Sound,
    macroquad::prelude::{Vec2, Texture2D, draw_texture_ex, WHITE, DrawTextureParams, Rect},
    text::MessagePage,
    gui::text::DynamicText,
    graphics::byte_texture,
    CRY_ID,
};

use crate::{
    Battle,
    transitions::{
        BattleTransition,
        BattleTransitionGui,
        BattleIntroduction,
    },
    gui::status::PokemonStatusGui,
};

static mut PLAYER: Option<Texture2D> = None;

pub struct BasicBattleIntroduction {

    alive: bool,
    finished: bool,

    pub text: DynamicText,
    
    player: Texture2D,
	counter: f32,
    offsets: (f32, f32),

    finished_panel: bool,

}

impl BasicBattleIntroduction {

    const OFFSETS: (f32, f32) = (-PokemonStatusGui::BATTLE_OFFSET, PokemonStatusGui::BATTLE_OFFSET);

    pub fn new(panel: Vec2, len: usize) -> Self {
        Self {
            alive: false,
            finished: false,

            text: DynamicText::new(Vec2::new(11.0, 11.0), panel, 1, game::text::TextColor::White, len, "btlintro"),

            player: unsafe { *PLAYER.get_or_insert(byte_texture(include_bytes!("../../../assets/player.png"))) },
			counter: 0.0,
            offsets: Self::OFFSETS, // opponent, player
            
            finished_panel: false,
        }
    }

    #[deprecated(note = "bad code, return vec of string (lines)")]
    pub fn concatenate(active: &Box<[crate::pokemon::ActivePokemon]>) -> String {
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

    pub fn common_setup(&mut self, active: &Box<[crate::pokemon::ActivePokemon]>) {        
        self.text.push(
            MessagePage::new(
                vec![
                    format!("Go! {}!", Self::concatenate(active)),
                ],
                Some(0.5),
            )
        );
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
            for active in battle.player.active.iter() {
                active.renderer.render(Vec2::new(offset, 0.0));
            }
        }
    }

}

impl BattleTransitionGui for BasicBattleIntroduction {

    fn input(&mut self) {
        self.text.input();
    }

}

impl BattleIntroduction for BasicBattleIntroduction {

    fn setup(&mut self, battle: &Battle) {
        self.text.clear();
        self.text.push(
            MessagePage::new(
                vec![
                    format!("Wild {} appeared!", Self::concatenate(&battle.opponent.active))
                ],
                None,
            )
        );
        self.common_setup(&battle.player.active);
    }

    fn update_gui(&mut self, delta: f32, battle: &mut Battle) {
        if battle.opponent.active[0].status.is_alive() {
            if self.offsets.0 != 0.0 {
                self.offsets.0 += delta * 240.0;
                if self.offsets.0 > 0.0 {
                    self.offsets.0 = 0.0;
                }
            }
        } else if self.text.can_continue() && self.text.current() >= self.text.len() - 2 {
            for active in battle.opponent.active.iter_mut() {
                active.status.spawn();
                if let Some(instance) = active.pokemon.as_ref() {
                    play_sound(&Sound::variant(CRY_ID, Some(instance.pokemon.data.id)));
                }
            }
            
        }

        if battle.player.active[0].status.is_alive() {
            if self.offsets.1 != 0.0 {
                self.offsets.1 -= delta * 240.0;
                if self.offsets.1 < 0.0 {
                    self.offsets.1 = 0.0;
                    self.finished_panel = true;
                }
            }
        } else if self.counter >= 104.0 {
            for active in battle.player.active.iter_mut() {
                active.status.spawn();
                if let Some(instance) = active.pokemon.as_ref() {
                    play_sound(&Sound::variant(CRY_ID, Some(instance.pokemon.data.id)));
                }
            }
        }
        
    }

    fn render_offset(&self, battle: &Battle, offset: f32) {
        for active in battle.opponent.active.iter() {
            active.renderer.render(Vec2::new(-offset, 0.0));
            active.status.render_offset(self.offsets.0, 0.0);
        }
        self.render_player(battle, offset);
    }

}

impl BattleTransition for BasicBattleIntroduction {

    fn on_start(&mut self) {}

    fn update(&mut self, delta: f32) {
        self.text.update(delta);
        if self.text.current() + 1 == self.text.len() {
            if self.counter < 104.0 {
                self.counter += delta * 180.0;                
            } else if self.text.is_finished() {
                self.text.despawn();
                self.finished = true;
            }
        }
	}

    fn render(&self) {
        self.text.render(#[cfg(debug_assertions)] "render");
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
        self.offsets = Self::OFFSETS;
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