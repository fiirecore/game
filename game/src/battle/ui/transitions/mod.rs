use crate::{battle_glue::BattleTrainerEntry, gui::TextDisplay, tetra::Context, util::Completable};

use crate::battle::{data::BattleType, pokemon::view::{BattlePartyKnown, BattlePartyUnknown}, ui::view::{ActivePokemonParty, ActiveRenderer}};

use crate::pokedex::moves::target::PlayerId;

pub mod managers;

pub mod transitions;
pub mod openers;
pub mod introductions;
pub mod closers;


pub(crate) trait BattleTransition: Completable {

    fn update(&mut self, ctx: &mut Context, delta: f32);

    fn draw(&self, ctx: &mut Context);

    // fn render_below_player(&self);

}

pub(crate) trait BattleOpener: Completable  {

    fn spawn(&mut self, trainer: Option<&BattleTrainerEntry>);
    
    fn update(&mut self, delta: f32);

    fn draw_below_panel(&self, ctx: &mut Context, player: &ActiveRenderer, opponent: &ActiveRenderer);

    fn draw(&self, ctx: &mut Context);

    fn offset(&self) -> f32;

}

pub(crate) trait BattleIntroduction: Completable {

    fn spawn(&mut self, battle_type: BattleType, trainer: Option<&BattleTrainerEntry>, player: &BattlePartyKnown, opponent: &BattlePartyUnknown, text: &mut TextDisplay);

    fn update(&mut self, ctx: &mut Context, delta: f32, player: &mut ActivePokemonParty<BattlePartyKnown>, opponent: &mut ActivePokemonParty<BattlePartyUnknown>, text: &mut TextDisplay);

    fn draw(&self, ctx: &mut Context, player: &ActiveRenderer, opponent: &ActiveRenderer);

}

pub(crate) trait BattleCloser: Completable {

    fn spawn(&mut self, winner: Option<&PlayerId>, trainer: Option<&BattleTrainerEntry>, text: &mut TextDisplay);
    
    fn update(&mut self, ctx: &mut Context, delta: f32, text: &mut TextDisplay);

    fn draw(&self, ctx: &mut Context);

    fn draw_battle(&self, ctx: &mut Context);

    fn world_active(&self) -> bool;

}