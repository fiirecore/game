use crate::{
    util::{Entity, Reset, Completable},
    text::MessagePage,
    audio::sound::Sound,
    gui::TextDisplay,
    graphics::{position, ZERO},
    tetra::{
        Context,
        graphics::{Texture, Rectangle, Color},
    },
    battle_glue::BattleTrainerEntry,
    CRY_ID, 
    play_sound,
};

use crate::battle::{
    data::BattleType,
    pokemon::{
        view::{
            BattlePartyView,
            BattlePartyKnown,
            BattlePartyUnknown,
            PokemonView,
        },
    },
    ui::{
        pokemon::PokemonStatusGui,
        transitions::BattleIntroduction,
        view::{
            ActivePokemonParty,
            ActiveRenderer,
        },
    },
};

pub struct BasicBattleIntroduction {
 
    player: Texture,
	counter: f32,
    offsets: (f32, f32),

}
impl BasicBattleIntroduction {

    const OFFSETS: (f32, f32) = (-PokemonStatusGui::BATTLE_OFFSET, PokemonStatusGui::BATTLE_OFFSET);

    const PLAYER_T1: f32 = 42.0;
    const PLAYER_T2: f32 = Self::PLAYER_T1 + 18.0;
    const PLAYER_T3: f32 = Self::PLAYER_T2 + 18.0;
    const PLAYER_DESPAWN: f32 = 104.0;

    pub fn new(ctx: &mut Context) -> Self {
        Self {
            player: super::super::openers::player_texture(ctx).clone(),
            counter: 0.0,
            offsets: Self::OFFSETS, // opponent, player
        }
    }

    #[deprecated(note = "bad code, return vec of string (lines)")]
    pub(crate) fn concatenate(party: &dyn BattlePartyView) -> String {
        let mut string = String::new();
        let len = party.active_len();
        for index in 0..len {
            if let Some(instance) = party.active(index) {
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

    pub(crate) fn common_setup(&mut self, text: &mut TextDisplay, party: &dyn BattlePartyView) {        
        text.push(
            MessagePage::new(
                vec![
                    format!("Go! {}!", Self::concatenate(party)),
                ],
                Some(0.5),
            )
        );
    }

    pub(crate) fn draw_player(&self, ctx: &mut Context, player: &ActiveRenderer) {
        if self.counter < Self::PLAYER_DESPAWN {
            self.player.draw_region(
                ctx, 
                Rectangle::new(
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
                ),
                position(41.0 + - self.counter, 49.0),
            )
        } else {
            for active in player.iter() {
                active.renderer.draw(ctx, ZERO, Color::WHITE);
            }
        }
    }

    pub(crate) fn draw_opponent(&self, ctx: &mut Context, opponent: &ActiveRenderer) {
        for active in opponent.iter() {
            active.renderer.draw(ctx, ZERO, Color::WHITE);
            active.status.draw(ctx, self.offsets.0, 0.0);
        }
    }

}

impl BattleIntroduction for BasicBattleIntroduction {

    fn spawn(&mut self, _: BattleType, _: Option<&BattleTrainerEntry>, player: &BattlePartyKnown, opponent: &BattlePartyUnknown, text: &mut TextDisplay) {
        text.clear();
        text.push(
            MessagePage::new(
                vec![
                    format!("Wild {} appeared!", Self::concatenate(opponent))
                ],
                None,
            )
        );
        self.common_setup(text, player);
    }

    fn update(&mut self, ctx: &mut Context, delta: f32, player: &mut ActivePokemonParty<BattlePartyKnown>, opponent: &mut ActivePokemonParty<BattlePartyUnknown>, text: &mut TextDisplay) {

        text.update(ctx, delta);

        if text.current() + 1 == text.len() && self.counter < Self::PLAYER_DESPAWN {
            self.counter += delta * 180.0;
        }

        if opponent.renderer[0].status.alive() {
            if self.offsets.0 != 0.0 {
                self.offsets.0 += delta * 240.0;
                if self.offsets.0 > 0.0 {
                    self.offsets.0 = 0.0;
                }
            }
        } else if text.can_continue() && text.current() >= text.len() - 2 {
            for active in opponent.renderer.iter_mut() {
                active.status.spawn();
            }
            for instance in opponent.party.active.iter().flat_map(|index| index.map(|i| &opponent.party.pokemon[i] as &dyn PokemonView)) {
                play_sound(ctx, &Sound::variant(CRY_ID, Some(*instance.pokemon().id())));
            }            
        }

        if player.renderer[0].renderer.spawner.spawning() {
            for active in player.renderer.iter_mut() {
                active.renderer.spawner.update(delta);
            }
        } else if player.renderer[0].status.alive() {
            if self.offsets.1 != 0.0 {
                self.offsets.1 -= delta * 240.0;
                if self.offsets.1 < 0.0 {
                    self.offsets.1 = 0.0;
                }
            }
        } else if self.counter >= Self::PLAYER_T2 {
            for active in player.renderer.iter_mut() {
                active.renderer.spawn();
                active.status.spawn();
            }
            for instance in player.party.active.iter().flat_map(|index| index.map(|i| &player.party.pokemon[i])) {
                play_sound(ctx, &Sound::variant(CRY_ID, Some(*instance.pokemon.id())));
            }
        }
        
    }

    fn draw(&self, ctx: &mut Context, player: &ActiveRenderer, opponent: &ActiveRenderer) {
        self.draw_opponent(ctx, opponent);
        self.draw_player(ctx, player);
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